use std::collections::HashMap;

use algorithm::buf::{Bt, BtMut};

use crate::HpResult;
use crate::ValueType;

use super::make_extension_error;
use super::{Buffer, ErrorKind, Value};

macro_rules! fail {
    ($expr:expr) => {
        return Err(::std::convert::From::from($expr))
    };
}

pub fn peek_type<B: Bt+BtMut>(buffer: &mut Buffer<B>) -> HpResult<ValueType> {
    Ok(ValueType::from(buffer.peek_u8()))
}

pub fn decode_type<B: Bt+BtMut>(buffer: &mut Buffer<B>) -> HpResult<ValueType> {
    Ok(ValueType::from(buffer.get_u8()))
}

pub fn decode_bool<B: Bt+BtMut>(buffer: &mut Buffer<B>, pattern: ValueType) -> HpResult<Value> {
    match pattern {
        ValueType::Bool => {
            let b = buffer.try_get_u8()?;
            Ok(Value::from(b == 1))
        }
        _ => {
            unreachable!("not other numbers");
        }
    }
}

pub fn decode_number<B: Bt+BtMut>(buffer: &mut Buffer<B>, pattern: ValueType) -> HpResult<Value> {
    match pattern {
        ValueType::U8 => {
            Ok(Value::from(buffer.try_get_u8()?))
        }
        ValueType::I8 => {
            Ok(Value::from(buffer.try_get_i8()?))
        }
        ValueType::U16 => {
            Ok(Value::from(buffer.try_get_u16()?))
        }
        ValueType::I16 => {
            Ok(Value::from(buffer.try_get_i16()?))
        }
        ValueType::U32 => {
            Ok(Value::from(buffer.try_get_u32()?))
        }
        ValueType::I32 => {
            Ok(Value::from(buffer.try_get_i32()?))
        }
        ValueType::U64 => {
            Ok(Value::from(buffer.try_get_u64()?))
        }
        ValueType::I64 => {
            Ok(Value::from(buffer.try_get_i64()?))
        }
        ValueType::Varint => decode_varint(buffer),
        ValueType::F32 => {
            Ok(Value::from(buffer.try_get_f32()?))
        }
        ValueType::F64 => {
            Ok(Value::from(buffer.try_get_f64()?))
        }
        _ => {
            unreachable!("not other numbers");
        }
    }
}

pub fn decode_varint<B: Bt>(buffer: &mut B) -> HpResult<Value> {
    let mut real = 0u64;
    let mut shl_num = 0;
    loop {
        let data = buffer.try_get_u8()?;
        let read = (data & 0x7F) as u64;
        if let Some(sread) = read.checked_shl(shl_num) {
            real += sread;
        } else {
            fail!((ErrorKind::ParseError, "too big varint"));
        }
        shl_num += 7;
        if (data & 0x80) == 0 {
            break;
        }
    }
    let is_left = real % 2 == 1;
    let val = if is_left {
        -((real / 2) as i64) - 1
    } else {
        (real / 2) as i64
    };
    Ok(Value::Varint(val))
}

pub fn decode_string<B: Bt>(buffer: &mut B) -> HpResult<String> {
    let len: u16 = decode_varint(buffer)?.into();
    if len == 0 {
        return Ok(String::new());
    }
    if buffer.remaining() < len as usize {
        fail!((ErrorKind::NoLeftSpaceError, "space error"));
    }
    let rv = buffer.advance_chunk(len as usize).to_vec();
    let val = String::from_utf8(rv);
    if val.is_err() {
        fail!((ErrorKind::StringFormatError, "string format error"));
    }
    Ok(val.ok().unwrap())
}

pub fn decode_str_raw<B: Bt+BtMut>(buffer: &mut Buffer<B>, pattern: ValueType) -> HpResult<Value> {
    match pattern {
        ValueType::Str => {
            let len: u16 = decode_varint(buffer)?.into();
            if len == 0 {
                return Ok(Value::from(String::new()));
            }
            if buffer.remaining() < len as usize {
                fail!((ErrorKind::NoLeftSpaceError, "space error"));
            }
            let rv = buffer.advance_chunk(len as usize).to_vec();
            let val = String::from_utf8(rv);
            if val.is_err() {
                fail!((ErrorKind::StringFormatError, "string format error"));
            }
            Ok(Value::from(val.ok().unwrap()))
        }
        ValueType::Raw => {
            let len: u16 = decode_varint(buffer)?.into();
            if len == 0 {
                return Ok(Value::from(Vec::<u8>::new()));
            }
            if buffer.remaining() < len as usize {
                fail!((ErrorKind::NoLeftSpaceError, "space error"));
            }
            let rv = buffer.advance_chunk(len as usize).to_vec();
            Ok(Value::from(rv))
        }
        _ => {
            unreachable!("not other str");
        }
    }
}

pub fn decode_map<B: Bt+BtMut>(buffer: &mut Buffer<B>) -> HpResult<Value> {
    let mut map = HashMap::<Value, Value>::new();
    let arr_len: u32 = decode_varint(buffer)?.into();
    for _ in 0..arr_len / 2 {
        let key = decode_field(buffer)?;
        let sub_value = decode_field(buffer)?;
        map.insert(key, sub_value);
    }
    Ok(Value::from(map))
}

pub fn decode_arr<B: Bt+BtMut>(buffer: &mut Buffer<B>) -> HpResult<Value> {
    let mut arr = Vec::<Value>::new();
    let arr_len: u16 = decode_varint(buffer)?.into();
    for _ in 0..arr_len {
        let sub_value = decode_field(buffer)?;
        arr.push(sub_value);
    }
    Ok(Value::from(arr))
}

pub fn decode_by_pattern<B: Bt+BtMut>(buffer: &mut Buffer<B>, pattern: &ValueType) -> HpResult<Value> {
    match *pattern {
        ValueType::Bool => decode_bool(buffer, *pattern),
        ValueType::U8
        | ValueType::I8
        | ValueType::U16
        | ValueType::I16
        | ValueType::U32
        | ValueType::I32 => decode_number(buffer, *pattern),
        ValueType::F32 => Ok(Value::F32(buffer.try_get_f32()?)),
        ValueType::F64 => Ok(Value::F64(buffer.try_get_f64()?)),
        ValueType::Varint => decode_varint(buffer),
        ValueType::Str | ValueType::Raw => decode_str_raw(buffer, *pattern),
        ValueType::Map => decode_map(buffer),
        ValueType::Arr => decode_arr(buffer),
        ValueType::StrIdx => {
            let idx: u16 = decode_varint(buffer)?.into();
            Ok(Value::from(buffer.get_str(idx)?))
        }
        // ValueType::Kv => {
        //     let name: String = decode_str_raw(buffer, *pattern)?.into();
        //     let len: u32 = decode_varint(buffer)?.into();
        //     let mut result = vec![];
        //     for _ in 0..len {
        //         result.push(decode_field(buffer)?);
        //     }
        //     Ok(Value::from((name, result)))
        // }
        // TYPE_AMAP => decode_array!(decode_field(buffer, config), Value::AMap, Value::Map),
        ValueType::Nil => Ok(Value::Nil),
        _ => fail!((ErrorKind::TypeNotMatchError, "must match type")),
    }
}

pub fn decode_field<B: Bt+BtMut>(buffer: &mut Buffer<B>) -> HpResult<Value> {
    let pattern = decode_type(buffer)?.into();
    decode_by_pattern(buffer, &pattern)
}

pub fn decode_proto<B: Bt+BtMut>(buffer: &mut Buffer<B>) -> HpResult<(String, Vec<Value>)> {
    let name = decode_str_raw(buffer, ValueType::Str)?.into();

    let str_len: u16 = decode_varint(buffer)?.into();
    for _ in 0..str_len {
        let value = decode_str_raw(buffer, ValueType::Str)?.into();
        buffer.add_str(value);
    }

    let sub_value = decode_field(buffer)?;
    match sub_value {
        Value::Arr(val) => Ok((name, val)),
        _ => Err(make_extension_error("proto is not array", None)),
    }
}

pub fn decode_msg<B: Bt+BtMut>(buffer: &mut Buffer<B>) -> HpResult<Vec<Value>> {
    let str_len: u16 = decode_varint(buffer)?.into();
    for _ in 0..str_len {
        let value = decode_str_raw(buffer, ValueType::Str)?.into();
        buffer.add_str(value);
    }

    let sub_value = decode_field(buffer)?;
    match sub_value {
        Value::Arr(val) => Ok(val),
        _ => Err(make_extension_error("proto is not array", None)),
    }
}


pub fn decode_msg_map<B: Bt+BtMut>(buffer: &mut Buffer<B>) -> HpResult<Value> {
    let str_len: u16 = decode_varint(buffer)?.into();
    for _ in 0..str_len {
        let value = decode_str_raw(buffer, ValueType::Str)?.into();
        buffer.add_str(value);
    }

    let sub_value = decode_map(buffer)?;
    match sub_value {
        Value::Map(_) => Ok(sub_value),
        _ => Err(make_extension_error("proto is not array", None)),
    }
}