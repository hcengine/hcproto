use std::collections::HashMap;

use crate::decode::{decode_field, decode_str_raw, decode_type, decode_varint};
use crate::error::HpError;
use crate::{Buffer, HpResult, Value, ValueType};
use serde::de::value::{MapAccessDeserializer, SeqAccessDeserializer};
use serde::de::{
    self, Deserialize, DeserializeSeed, EnumAccess, IntoDeserializer, MapAccess, SeqAccess,
    VariantAccess, Visitor,
};
use serde::forward_to_deserialize_any;

#[derive(Debug)]
pub struct Deserializer<'de> {
    buf: &'de mut Buffer,
}

impl<'de> Deserializer<'de> {
    pub fn new(buf: &'de mut Buffer) -> HpResult<Self> {
        let str_len: u16 = decode_varint(buf)?.into();
        for _ in 0..str_len {
            let value = decode_str_raw(buf, ValueType::Str)?.into();
            buf.add_str(value);
        }
        Ok(Deserializer { buf })
    }

    fn visit_val<V>(
        &mut self,
        value: Value,
        visitor: V,
    ) -> Result<V::Value, HpError>
    where
        V: Visitor<'de>, {
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
            _ => unreachable!()
        }
    }

    fn visit_seq<V>(
        &mut self,
        val: Vec<Value>,
        visitor: V,
    ) -> Result<V::Value, HpError>
    where
        V: Visitor<'de>,
    {
        todo!()
        // let value = visitor.visit_seq(DeserializerSeqVisitor { de: self, len, end })?;
        // assert_next_token(self, end)?;
        // Ok(value)
    }

    fn visit_map<V>(
        &mut self,
        val: HashMap<Value, Value>,
        visitor: V,
    ) -> Result<V::Value, HpError>
    where
        V: Visitor<'de>,
    {
        todo!()
        // let value = visitor.visit_map(DeserializerMapVisitor { de: self, len, end })?;
        // assert_next_token(self, end)?;
        // Ok(value)
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
        let value = decode_field(&mut self.buf)?;
        self.visit_val(value, visitor)
        // match value {
        //     crate::Value::Nil => visitor.visit_none(),
        //     crate::Value::Bool(v) => visitor.visit_bool(v),
        //     crate::Value::U8(v) => visitor.visit_u8(v),
        //     crate::Value::I8(v) => visitor.visit_i8(v),
        //     crate::Value::U16(v) => visitor.visit_u16(v),
        //     crate::Value::I16(v) => visitor.visit_i16(v),
        //     crate::Value::U32(v) => visitor.visit_u32(v),
        //     crate::Value::I32(v) => visitor.visit_i32(v),
        //     crate::Value::U64(v) => visitor.visit_u64(v),
        //     crate::Value::I64(v) => visitor.visit_i64(v),
        //     crate::Value::Varint(v) => visitor.visit_i64(v),
        //     crate::Value::F32(v) => visitor.visit_f32(v),
        //     crate::Value::F64(v) => visitor.visit_f64(v),
        //     crate::Value::Str(v) => visitor.visit_string(v),
        //     crate::Value::Raw(vec) => visitor.visit_byte_buf(vec),
        //     crate::Value::Arr(vec) => self.visit_seq(vec, visitor),
        //     crate::Value::Map(hash_map) => self.visit_map(hash_map, visitor),
        //     // crate::Value::KeyValue(name, vec) => todo!(),
        //     _ => unreachable!()
        // }
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
            Err(de::Error::custom(
                "unit struct must be none",
            ))
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
        let t = decode_type(&mut self.buf)?;
        let value = decode_field(&mut self.buf)?;
        match value {
            Value::KeyValue(name, value) => {
                
                Err(de::Error::custom(
                    "struct must be kv type",
                ))
            }
            _ => {
                Err(de::Error::custom(
                    "struct must be kv type",
                ))
            }
        }
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    
}
