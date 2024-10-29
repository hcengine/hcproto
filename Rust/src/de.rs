use std::collections::HashMap;


use crate::decode::{
    decode_field, decode_str_raw, decode_type, decode_varint, peek_type,
};
use crate::error::HpError;
use crate::{Buffer, HpResult, Value, ValueType};

use serde::de::{
    self, Deserialize, DeserializeSeed, EnumAccess, Error, MapAccess, SeqAccess,
    VariantAccess, Visitor,
};
use serde::{forward_to_deserialize_any};

pub fn from_buffer<'a, T>(buf: &'a mut Buffer) -> HpResult<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::new(buf)?;
    let t = T::deserialize(&mut deserializer)?;
    if deserializer.buf.buf.is_empty() {
        Ok(t)
    } else {
        Err(HpError::custom("left buffer"))
    }
}

#[derive(Debug)]
pub struct Deserializer<'de> {
    buf: &'de mut Buffer,
    value: Option<Value>,
}

impl<'de> Deserializer<'de> {
    pub fn new(buf: &'de mut Buffer) -> HpResult<Self> {
        let str_len: u16 = decode_varint(buf)?.into();
        for _ in 0..str_len {
            let value = decode_str_raw(buf, ValueType::Str)?.into();
            buf.add_str(value);
        }
        Ok(Deserializer { buf, value: None })
    }

    fn visit_val<V>(&mut self, value: Value, visitor: V) -> Result<V::Value, HpError>
    where
        V: Visitor<'de>,
    {
        match value {
            crate::Value::Nil => visitor.visit_none(),
            crate::Value::Bool(v) => visitor.visit_bool(v),
            crate::Value::U8(v) => visitor.visit_u8(v),
            crate::Value::I8(v) => visitor.visit_i8(v),
            crate::Value::U16(v) => visitor.visit_u16(v),
            crate::Value::I16(v) => visitor.visit_i16(v),
            crate::Value::U32(v) => visitor.visit_u32(v),
            crate::Value::I32(v) => visitor.visit_i32(v),
            crate::Value::U64(v) => visitor.visit_u64(v),
            crate::Value::I64(v) => visitor.visit_i64(v),
            crate::Value::Varint(v) => visitor.visit_i64(v),
            crate::Value::F32(v) => visitor.visit_f32(v),
            crate::Value::F64(v) => visitor.visit_f64(v),
            crate::Value::Str(v) => visitor.visit_string(v),
            crate::Value::Raw(vec) => visitor.visit_byte_buf(vec),
            crate::Value::Arr(vec) => self.visit_seq(vec, visitor),
            crate::Value::Map(hash_map) => self.visit_map(hash_map, visitor),
            // crate::Value::KeyValue(name, vec) => todo!(),
            _ => unreachable!(),
        }
    }

    fn visit_seq<V>(&mut self, mut val: Vec<Value>, visitor: V) -> Result<V::Value, HpError>
    where
        V: Visitor<'de>,
    {
        let array = val.drain(..).rev().collect();
        let value = visitor.visit_seq(CommaSeparated {de: self, array, len: 0})?;
        Ok(value)
    }

    fn visit_map<V>(&mut self, val: HashMap<Value, Value>, visitor: V) -> Result<V::Value, HpError>
    where
        V: Visitor<'de>,
    {
        let mut array = vec![];
        for (k, v) in val.into_iter() {
            array.push(k);
            array.push(v);
        }
        let array = array.drain(..).rev().collect();
        let value = visitor.visit_map(CommaSeparated {de: self, array, len: 0})?;
        Ok(value)
    }
}

impl<'de> de::Deserializer<'de> for &mut Deserializer<'de> {
    type Error = HpError;

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf unit seq map identifier ignored_any
    }

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.value.is_some() {
            let value = self.value.take().unwrap();
            self.visit_val(value, visitor)
        } else {
            let value = decode_field(&mut self.buf)?;
            self.visit_val(value, visitor)
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let value = decode_field(&mut self.buf)?;
        if value.is_nil() {
            visitor.visit_none()
        } else {
            self.visit_val(value, visitor)
        }
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let value = decode_field(&mut self.buf)?;
        if value.is_nil() {
            visitor.visit_none()
        } else {
            Err(de::Error::custom("unit struct must be none"))
        }
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let t = peek_type(&mut self.buf)?;
        if t == ValueType::Arr {
            visitor.visit_newtype_struct(self)
        } else {
            Err(de::Error::custom("struct must be kv type"))
        }
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let value = decode_field(&mut self.buf)?;
        match value {
            Value::Arr(v) => {
                self.visit_seq(v, visitor)
            }
            _ => {
                Err(de::Error::custom("struct must be kv type"))
            }
        }
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_tuple(len, visitor)
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let t = decode_type(&mut self.buf)?;
        if t == ValueType::Arr {
            let len: u32 = decode_varint(&mut self.buf)?.into();
            visitor.visit_map(CommaSeparated {
                de: self,
                array: vec![],
                len: len as usize,
            })
        } else {
            Err(de::Error::custom("struct must be kv type"))
        }
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }
}

struct CommaSeparated<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
    array: Vec<Value>,
    len: usize,
}

// `SeqAccess` is provided to the `Visitor` to give it the ability to iterate
// through elements of the sequence.
impl<'de> SeqAccess<'de> for CommaSeparated<'_, 'de> {
    type Error = HpError;

    fn next_element_seed<T>(&mut self, seed: T) -> HpResult<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        if !self.array.is_empty() {
            self.de.value = self.array.pop();
            seed.deserialize(&mut *self.de).map(Some)
        } else {
            if self.len == 0 {
                return Ok(None);
            }
            seed.deserialize(&mut *self.de).map(Some)
        }
    }
}

impl<'de> MapAccess<'de> for CommaSeparated<'_, 'de> {
    type Error = HpError;

    fn next_key_seed<K>(&mut self, seed: K) -> HpResult<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        if !self.array.is_empty() {
            self.de.value = self.array.pop();
            seed.deserialize(&mut *self.de).map(Some)
        } else {
            if self.len == 0 {
                return Ok(None);
            }
            self.len -= 1;
            seed.deserialize(&mut *self.de).map(Some)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> HpResult<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        if !self.array.is_empty() {
            self.de.value = self.array.pop();
            seed.deserialize(&mut *self.de)
        } else {
            seed.deserialize(&mut *self.de)
        }
    }
}


struct Enum<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
}

impl<'a, 'de> Enum<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>) -> Self {
        Enum { de }
    }
}

impl<'de> EnumAccess<'de> for Enum<'_, 'de> {
    type Error = HpError;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> HpResult<(V::Value, Self::Variant)>
    where
        V: DeserializeSeed<'de>,
    {
        let val = seed.deserialize(&mut *self.de)?;
        Ok((val, self))
    }
}

// `VariantAccess` is provided to the `Visitor` to give it the ability to see
// the content of the single variant that it decided to deserialize.
impl<'de> VariantAccess<'de> for Enum<'_, 'de> {
    type Error = HpError;

    // If the `Visitor` expected this variant to be a unit variant, the input
    // should have been the plain string case handled in `deserialize_enum`.
    fn unit_variant(self) -> HpResult<()> {
        Err(Error::custom("unint variant"))
    }

    // Newtype variants are represented in JSON as `{ NAME: VALUE }` so
    // deserialize the value here.
    fn newtype_variant_seed<T>(self, seed: T) -> HpResult<T::Value>
    where
        T: DeserializeSeed<'de>,
    {
        seed.deserialize(self.de)
    }

    // Tuple variants are represented in JSON as `{ NAME: [DATA...] }` so
    // deserialize the sequence of data here.
    fn tuple_variant<V>(self, _len: usize, visitor: V) -> HpResult<V::Value>
    where
        V: Visitor<'de>,
    {
        de::Deserializer::deserialize_seq(self.de, visitor)
    }

    // Struct variants are represented in JSON as `{ NAME: { K: V, ... } }` so
    // deserialize the inner map here.
    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> HpResult<V::Value>
    where
        V: Visitor<'de>,
    {
        de::Deserializer::deserialize_map(self.de, visitor)
    }
}
