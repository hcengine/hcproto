use std::collections::HashMap;
use std::io::Read;
use std::mem;

use algorithm::buf::Bt;

use crate::ValueType;
use crate::{HpResult, ValueType::StrIdx, ValueType::Varint};

use super::make_extension_error;
use super::{Buffer, ErrorKind, Value};

macro_rules! fail {
    ($expr:expr) => {
        return Err(::std::convert::From::from($expr))
    };
}

macro_rules! try_read {
    ($expr:expr, $val:expr) => {{
        if $expr? != $val {
            fail!((
                crate::ErrorKind::NoLeftSpaceError,
                "must left space to read "
            ));
        }
    }};
}


pub fn peek_type(buffer: &mut Buffer) -> HpResult<ValueType> {
    Ok(ValueType::from(buffer.buf.peek_u8()))
}

pub fn decode_type(buffer: &mut Buffer) -> HpResult<ValueType> {
    Ok(ValueType::from(buffer.buf.get_u8()))
}

pub fn decode_bool(buffer: &mut Buffer, pattern: ValueType) -> HpResult<Value> {
    match pattern {
        ValueType::Bool => {
            let data: &mut [u8; 1] = &mut [0];
            try_read!(buffer.read(data), data.len());
            Ok(Value::from(if data[0] == 1 { true } else { false }))
        }
        _ => {
            unreachable!("not other numbers");
        }
    }
}

pub fn decode_number(buffer: &mut Buffer, pattern: ValueType) -> HpResult<Value> {
    match pattern {
        ValueType::U8 => {
            let data: &mut [u8; 1] = &mut [0];
            try_read!(buffer.read(data), data.len());
            Ok(Value::from(data[0]))
        }
        ValueType::I8 => {
            let data: &mut [u8; 1] = &mut [0];
            try_read!(buffer.read(data), data.len());
            Ok(Value::from(data[0] as i8))
        }
        ValueType::U16 => {
            let data: &mut [u8; 2] = &mut [0, 0];
            try_read!(buffer.read(data), data.len());
            let val = unsafe { mem::transmute::<[u8; 2], u16>(*data) };
            Ok(Value::from(u16::from_le(val)))
        }
        ValueType::I16 => {
            let data: &mut [u8; 2] = &mut [0, 0];
            try_read!(buffer.read(data), data.len());
            let val = unsafe { mem::transmute::<[u8; 2], i16>(*data) };
            Ok(Value::from(i16::from_le(val)))
        }
        ValueType::U32 => {
            let data: &mut [u8; 4] = &mut [0, 0, 0, 0];
            try_read!(buffer.read(data), data.len());
            let val = unsafe { mem::transmute::<[u8; 4], u32>(*data) };
            Ok(Value::from(u32::from_le(val)))
        }
        ValueType::I32 => {
            let data: &mut [u8; 4] = &mut [0, 0, 0, 0];
            try_read!(buffer.read(data), data.len());
            let val = unsafe { mem::transmute::<[u8; 4], i32>(*data) };
            Ok(Value::from(i32::from_le(val)))
        }
        ValueType::U64 => {
            let data: &mut [u8; 8] = &mut [0, 0, 0, 0, 0, 0, 0, 0];
            try_read!(buffer.read(data), data.len());
            let val = unsafe { mem::transmute::<[u8; 8], u64>(*data) };
            Ok(Value::from(u64::from_le(val)))
        }
        ValueType::I64 => {
            let data: &mut [u8; 8] = &mut [0, 0, 0, 0, 0, 0, 0, 0];
            try_read!(buffer.read(data), data.len());
            let val = unsafe { mem::transmute::<[u8; 8], i64>(*data) };
            Ok(Value::from(i64::from_le(val)))
        }
        ValueType::Varint => decode_varint(buffer),
        ValueType::F32 => {
            let data: &mut [u8; 4] = &mut [0, 0, 0, 0];
            try_read!(buffer.read(data), data.len());
            let val = unsafe { mem::transmute::<[u8; 4], i32>(*data) };
            Ok(Value::from(val as f32 / 1000.0))
        }
        ValueType::F64 => {
            let data: &mut [u8; 8] = &mut [0, 0, 0, 0, 0, 0, 0, 0];
            try_read!(buffer.read(data), data.len());
            let val = unsafe { mem::transmute::<[u8; 8], i64>(*data) };
            Ok(Value::from(val as f64 / 1000000.0))
        }
        _ => {
            unreachable!("not other numbers");
        }
    }
}

pub fn decode_varint(buffer: &mut Buffer) -> HpResult<Value> {
    let data: &mut [u8; 1] = &mut [0];
    let mut real = 0u64;
    let mut shl_num = 0;
    loop {
        try_read!(buffer.read(data), data.len());
        let read = (data[0] & 0x7F) as u64;
        if let Some(sread) = read.checked_shl(shl_num) {
            real += sread;
        } else {
            fail!((ErrorKind::ParseError, "too big varint"));
        }
        shl_num += 7;
        if (data[0] & 0x80) == 0 {
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

pub fn decode_str_raw(buffer: &mut Buffer, pattern: ValueType) -> HpResult<Value> {
    match pattern {
        ValueType::Str => {
            let len: u16 = decode_varint(buffer)?.into();
            if len == 0 {
                return Ok(Value::from(String::new()));
            }
            let mut rv = vec![0; len as usize];
            try_read!(buffer.read(&mut rv[..]), len as usize);
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
            let mut rv = vec![0; len as usize];
            try_read!(buffer.read(&mut rv[..]), len as usize);
            Ok(Value::from(rv))
        }
        _ => {
            unreachable!("not other str");
        }
    }
}

pub fn decode_map(buffer: &mut Buffer) -> HpResult<Value> {
    let mut map = HashMap::<Value, Value>::new();
    let arr_len: u16 = decode_varint(buffer)?.into();
    for _ in 0..arr_len {
        let key = decode_field(buffer)?;
        let sub_value = decode_field(buffer)?;
        map.insert(key, sub_value);
    }
    Ok(Value::from(map))
}

pub fn decode_arr(buffer: &mut Buffer) -> HpResult<Value> {
    let mut arr = Vec::<Value>::new();
    let arr_len: u16 = decode_varint(buffer)?.into();
    for _ in 0..arr_len {
        let sub_value = decode_field(buffer)?;
        arr.push(sub_value);
    }
    Ok(Value::from(arr))
}

pub fn decode_by_pattern(buffer: &mut Buffer, pattern: &ValueType) -> HpResult<Value> {
    match *pattern {
        ValueType::Bool => decode_bool(buffer, *pattern),
        ValueType::U8 | ValueType::I8 | ValueType::U16 | ValueType::I16 | ValueType::U32 | ValueType::I32 => {
            decode_number(buffer, *pattern)
        }
        ValueType::F32 => {
            let val: i64 = decode_varint(buffer)?.into();
            Ok(Value::F32(val as f32 / 1000.0))
        }
        ValueType::F64 => {
            let val: i64 = decode_varint(buffer)?.into();
            Ok(Value::F64(val as f64 / 1000000.0f64))
        }
        ValueType::Varint => decode_varint(buffer),
        ValueType::Str | ValueType::Raw => decode_str_raw(buffer, *pattern),
        ValueType::Map => decode_map(buffer),
        ValueType::Arr => decode_arr(buffer),
        ValueType::StrIdx => {
            let idx: u16 = decode_varint(buffer)?.into();
            Ok(Value::from(buffer.get_str(idx)?))
        }
        ValueType::Kv => {
            let name: String = decode_str_raw(buffer, *pattern)?.into();
            let len: u32 = decode_varint(buffer)?.into();
            let mut result = vec![];
            for _ in 0..len {
                result.push(decode_field(buffer)?);
            }
            Ok(Value::from((name, result)))
        }
        // TYPE_AMAP => decode_array!(decode_field(buffer, config), Value::AMap, Value::Map),
        ValueType::Nil => Ok(Value::Nil),
        _ => fail!((ErrorKind::TypeNotMatchError, "must match type")),
    }
}

pub fn decode_field(buffer: &mut Buffer) -> HpResult<Value> {
    let pattern = decode_type(buffer)?.into();
    decode_by_pattern(buffer, &pattern)
}

pub fn decode_proto(buffer: &mut Buffer) -> HpResult<(String, Vec<Value>)> {
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
