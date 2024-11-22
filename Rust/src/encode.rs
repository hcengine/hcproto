use std::io::Result;

use algorithm::buf::{Bt, BtMut};

use crate::{get_type_by_value, Buffer, Value, ValueType};

#[inline(always)]
pub fn append_and_align<B: BtMut>(buffer: &mut B, val: &[u8]) -> Result<()> {
    let _add = match val.len() % 2 {
        0 => 0,
        val => 2 - val,
    };
    buffer.put_slice(val);
    Ok(())
}

#[inline(always)]
pub fn encode_sure_type<B: Bt + BtMut>(buffer: &mut Buffer<B>, value: ValueType) -> Result<()> {
    buffer.put_u8(value as u8);
    Ok(())
}

#[inline(always)]
pub fn encode_type<B: Bt + BtMut>(buffer: &mut Buffer<B>, value: &Value) -> Result<()> {
    buffer.put_u8(get_type_by_value(value) as u8);
    Ok(())
}

#[inline(always)]
pub fn encode_bool<B: Bt + BtMut>(buffer: &mut Buffer<B>, value: &Value) -> Result<()> {
    match *value {
        Value::Bool(val) => {
            buffer.put_u8(if val { 1 } else { 0 });
        }
        _ => unreachable!("encode_number only"),
    }
    Ok(())
}

#[inline(always)]
pub fn encode_number<B: Bt + BtMut>(buffer: &mut Buffer<B>, value: &Value) -> Result<()> {
    match *value {
        Value::U8(val) => {
            buffer.put_u8(val);
        }
        Value::I8(val) => {
            buffer.put_i8(val);
        }
        Value::U16(val) => {
            buffer.put_u16(val);
        }
        Value::I16(val) => {
            buffer.put_i16(val);
        }
        Value::U32(val) => {
            buffer.put_u32(val);
        }
        Value::I32(val) => {
            buffer.put_i32(val);
        }
        Value::U64(val) => {
            buffer.put_u64(val);
        }
        Value::I64(val) => {
            buffer.put_i64(val);
        }
        Value::F32(val) => {
            buffer.put_f32(val);
        }
        Value::F64(val) => {
            buffer.put_f64(val);
        }
        _ => unreachable!("encode_number only"),
    }
    Ok(())
}

#[inline(always)]
pub fn encode_varint<B: BtMut>(buffer: &mut B, value: &Value) -> Result<()> {
    let val = match *value {
        Value::U8(val) => val as i64,
        Value::I8(val) => val as i64,
        Value::U16(val) => val as i64,
        Value::I16(val) => val as i64,
        Value::U32(val) => val as i64,
        Value::I32(val) => val as i64,
        Value::U64(val) => val as i64,
        Value::I64(val) => val as i64,
        Value::Varint(val) => val as i64,
        _ => unreachable!("encode_number only"),
    };
    let mut real = if val < 0 {
        (-(val + 1)) as u64 * 2 + 1
    } else {
        (val as u64) * 2
    };
    loop {
        let data = (real & 0x7F) as u8;
        real = real >> 7;
        if real == 0 {
            buffer.put_u8(data);
            break;
        } else {
            buffer.put_u8(data | 0x80);
        }
    }
    Ok(())
}

#[inline(always)]
pub fn encode_str_idx<B: Bt + BtMut>(buffer: &mut Buffer<B>, pattern: &str) -> Result<()> {
    let idx = buffer.add_str(pattern.to_string());
    encode_sure_type(buffer, ValueType::StrIdx)?;
    encode_varint(buffer, &Value::U16(idx))?;
    Ok(())
}

// #[inline(always)]
// pub fn encode_str_idx_not_type(buffer: &mut Buffer, pattern: &str) -> Result<()> {
//     let idx = buffer.add_str(pattern.to_string());
//     encode_varint(buffer, &Value::U16(idx))?;
//     Ok(())
// }


pub fn encode_string<B: BtMut>(buffer: &mut B, val: &str) -> Result<()> {
    encode_varint(buffer, &Value::U16(val.as_bytes().len() as u16))?;
    append_and_align(buffer, &val.as_bytes()[..])?;
    Ok(())
}

#[inline(always)]
pub fn encode_str_raw<B: Bt + BtMut>(buffer: &mut Buffer<B>, value: &Value) -> Result<()> {
    match *value {
        Value::Str(ref val) => {
            encode_varint(buffer, &Value::U16(val.as_bytes().len() as u16))?;
            append_and_align(buffer, &val.as_bytes()[..])?;
        }
        Value::Raw(ref val) => {
            encode_varint(buffer, &Value::U16(val.len() as u16))?;
            append_and_align(buffer, &val[..])?;
        }
        _ => unreachable!("encode_str_raw only"),
    }
    Ok(())
}

pub fn encode_map<B: Bt + BtMut>(buffer: &mut Buffer<B>, value: &Value) -> Result<()> {
    match *value {
        Value::Map(ref val) => {
            encode_varint(buffer, &Value::from(val.len() as u16))?;
            for (name, sub_value) in val {
                encode_field(buffer, name)?;
                encode_field(buffer, sub_value)?;
            }
        }
        _ => unreachable!("encode_map only"),
    }
    Ok(())
}

pub fn encode_field<B: Bt + BtMut>(buffer: &mut Buffer<B>, value: &Value) -> Result<()> {
    match &*value {
        Value::Bool(_) => {
            encode_type(buffer, value)?;
            encode_bool(buffer, value)?;
        }
        Value::U8(_) | Value::I8(_) => {
            encode_type(buffer, value)?;
            encode_number(buffer, value)?;
        }
        Value::U16(_)
        | Value::I16(_)
        | Value::U32(_)
        | Value::I32(_)
        | Value::U64(_)
        | Value::I64(_)
        | Value::Varint(_) => {
            encode_sure_type(buffer, ValueType::Varint)?;
            encode_varint(buffer, value)?;
        }
        Value::F32(v) => {
            encode_sure_type(buffer, ValueType::F32)?;
            buffer.buf.put_f32(*v);
        }
        Value::F64(v) => {
            encode_sure_type(buffer, ValueType::F64)?;
            buffer.buf.put_f64(*v);
        }
        Value::Str(ref pattern) => {
            encode_str_idx(buffer, pattern)?;
        }
        Value::Raw(_) => {
            encode_type(buffer, value)?;
            encode_str_raw(buffer, value)?;
        }
        Value::Arr(ref val) => {
            encode_type(buffer, value)?;
            encode_varint(buffer, &Value::from(val.len() as u16))?;
            for v in val {
                encode_field(buffer, v)?;
            }
        }
        Value::Map(_) => {
            encode_type(buffer, value)?;
            encode_map(buffer, value)?;
        }
        // Value::Kv(ref key, ref val) => {
        //     encode_type(buffer, value)?;
        //     encode_str_idx_not_type(buffer, key)?;
        //     encode_varint(buffer, &Value::from(val.len() as u16))?;
        //     for v in val {
        //         encode_field(buffer, v)?;
        //     }
        // }
        Value::Nil => {
            encode_type(buffer, value)?;
        }
    }
    Ok(())
}

pub fn encode_proto<B: Bt + BtMut>(
    buffer: &mut Buffer<B>,
    name: &String,
    infos: Vec<Value>,
) -> Result<()> {
    let mut sub_buffer = Buffer::new();
    encode_field(&mut sub_buffer, &Value::from(infos))?;

    encode_str_raw(buffer, &Value::Str(name.clone()))?;
    encode_varint(buffer, &Value::U16(sub_buffer.str_arr.len() as u16))?;
    for v in &sub_buffer.str_arr {
        encode_str_raw(buffer, &Value::Str(v.to_string()))?;
    }

    // buffer.buf.extend(&sub_buffer)?;
    Ok(())
}

pub fn encode_msg<B: Bt + BtMut>(buffer: &mut Buffer<B>, infos: Vec<Value>) -> Result<()> {
    let mut sub_buffer = Buffer::new();
    encode_field(&mut sub_buffer, &Value::from(infos))?;

    encode_varint(buffer, &Value::U16(sub_buffer.str_arr.len() as u16))?;
    println!("buffer 1 = {:?}", buffer.chunk());
    for v in &sub_buffer.str_arr {
        encode_str_raw(buffer, &Value::Str(v.to_string()))?;
    }
    println!("buffer 2 = {:?}", buffer.chunk());
    println!("buffer zz = {:?}", buffer.buf.chunk());
    println!("buffer 4 = {:?}", sub_buffer.chunk());
    buffer.put_slice(sub_buffer.chunk());
    println!("buffer 3 = {:?}", buffer.chunk());
    Ok(())
}
