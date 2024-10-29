use crate::{
    encode::{
        append_and_align, encode_bool, encode_number, encode_str_idx, encode_sure_type, encode_type, encode_varint
    }, Buffer, HpError, HpResult, Value, ValueType::{self}
};
use serde::ser::{self, Serialize};

pub fn to_buffer<T>(value: &T) -> HpResult<Buffer>
where
    T: Serialize,
{
    let mut serializer = Serializer {
        buf: Buffer::new(),
    };
    value.serialize(&mut serializer)?;
    
    serializer.buf.export()
}

#[derive(Debug)]
pub struct Serializer {
    buf: Buffer,
}

impl Serializer {
    /// Creates the serializer.
    pub fn new(buf: Buffer) -> Self {
        Serializer { buf }
    }
}

impl<'s> ser::Serializer for &'s mut Serializer {
    type Ok = ();
    type Error = HpError;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        encode_sure_type(&mut self.buf, ValueType::Bool)?;
        encode_bool(&mut self.buf, &Value::Bool(v))?;
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        encode_sure_type(&mut self.buf, ValueType::I8)?;
        encode_number(&mut self.buf, &Value::I8(v))?;
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        encode_sure_type(&mut self.buf, ValueType::Varint)?;
        encode_varint(&mut self.buf, &Value::I16(v))?;
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        encode_sure_type(&mut self.buf, ValueType::Varint)?;
        encode_varint(&mut self.buf, &Value::I32(v))?;
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        encode_sure_type(&mut self.buf, ValueType::Varint)?;
        encode_varint(&mut self.buf, &Value::I64(v))?;
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        encode_sure_type(&mut self.buf, ValueType::U8)?;
        encode_number(&mut self.buf, &Value::U8(v))?;
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        encode_sure_type(&mut self.buf, ValueType::Varint)?;
        encode_varint(&mut self.buf, &Value::U16(v))?;
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        encode_sure_type(&mut self.buf, ValueType::Varint)?;
        encode_varint(&mut self.buf, &Value::U32(v))?;
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        encode_sure_type(&mut self.buf, ValueType::Varint)?;
        encode_varint(&mut self.buf, &Value::U64(v))?;
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        encode_number(&mut self.buf, &Value::F32(v))?;
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        encode_number(&mut self.buf, &Value::F64(v))?;
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        encode_str_idx(&mut self.buf, &v.to_string())?;
        Ok(())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        encode_str_idx(&mut self.buf, &v)?;
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        encode_sure_type(&mut self.buf, ValueType::Raw)?;
        encode_varint(&mut self.buf, &Value::U16(v.len() as u16))?;
        append_and_align(&mut self.buf, v)?;
        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        encode_type(&mut self.buf, &Value::Nil)?;
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        // encode_sure_type(&mut self.buf, ValueType::Kv)?;
        // encode_str_idx(&mut self.buf, &name)?;
        // encode_varint(&mut self.buf, &Value::from(1))?;
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        encode_sure_type(&mut self.buf, ValueType::Arr)?;
        // encode_str_idx_not_type(&mut self.buf, &variant)?;
        encode_varint(&mut self.buf, &Value::from(1))?;
        value.serialize(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        encode_sure_type(&mut self.buf, ValueType::Arr)?;
        encode_varint(&mut self.buf, &Value::from(len.unwrap_or(0) as u32))?;
        Ok(self)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.serialize_tuple_struct(variant, len)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self, Self::Error> {
        encode_sure_type(&mut self.buf, ValueType::Map)?;
        encode_varint(&mut self.buf, &Value::from(len.unwrap_or(0) as u16))?;
        Ok(self)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        encode_sure_type(&mut self.buf, ValueType::Arr)?;
        // encode_str_idx_not_type(&mut self.buf, &name)?;
        encode_varint(&mut self.buf, &Value::from(len as u32))?;
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(ser::Error::custom(
            "serialize_struct_variant is not supported",
        ))
    }

    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
        let _ = v;
        Err(ser::Error::custom("i128 is not supported"))
    }

    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        let _ = v;
        Err(ser::Error::custom("u128 is not supported"))
    }

    fn is_human_readable(&self) -> bool {
        false
    }
}

impl ser::SerializeSeq for &mut Serializer {
    type Ok = ();
    type Error = HpError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl ser::SerializeTuple for &mut Serializer {
    type Ok = ();
    type Error = HpError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl ser::SerializeTupleStruct for &mut Serializer {
    type Ok = ();
    type Error = HpError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl<'s> ser::SerializeTupleVariant for &mut Serializer {
    type Ok = ();
    type Error = HpError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl ser::SerializeMap for &mut Serializer {
    type Ok = ();
    type Error = HpError;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        key.serialize(&mut **self)
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl ser::SerializeStruct for &mut Serializer {
    type Ok = ();
    type Error = HpError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        key.serialize(&mut **self)?;
        value.serialize(&mut **self)?;
        Ok(())
    }

    fn end(self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl<'s> ser::SerializeStructVariant for &mut Serializer {
    type Ok = ();
    type Error = HpError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        key.serialize(&mut **self)?;
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<(), Self::Error> {
        Ok(())
    }
}
