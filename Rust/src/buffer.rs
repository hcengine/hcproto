use std::{
    collections::HashMap,
    io::{Read, Write},
};

use algorithm::buf::{BinaryMut, Bt};

use crate::{encode::{encode_str_raw, encode_varint}, error::HpResult, ErrorKind, HpError, Value};

#[derive(Debug)]
pub struct Buffer {
    pub buf: BinaryMut,
    pub str_arr: Vec<String>,
    pub str_map: HashMap<String, u16>,
}

impl Buffer {
    pub fn new() -> Buffer {
        Buffer {
            buf: BinaryMut::new(),
            str_arr: Vec::new(),
            str_map: HashMap::new(),
        }
    }

    pub fn add_str(&mut self, value: String) -> u16 {
        if self.str_map.contains_key(&value) {
            self.str_map[&value]
        } else {
            self.str_arr.push(value.clone());
            self.str_map.insert(value, self.str_arr.len() as u16 - 1);
            self.str_arr.len() as u16 - 1
        }
    }

    pub fn get_str(&self, idx: u16) -> HpResult<String> {
        if idx as usize >= self.str_arr.len() {
            Err(HpError::from((ErrorKind::BufferOverMaxError, "must left space to read ")))
        } else {
            Ok(self.str_arr[idx as usize].clone())
        }
    }

    pub fn export(self) -> HpResult<Buffer> {
        let mut sub_buffer = Buffer::new();
        encode_varint(&mut sub_buffer, &Value::U16(self.str_arr.len() as u16))?;
        for v in &self.str_arr {
            encode_str_raw(&mut sub_buffer, &Value::Str(v.to_string()))?;
        }
        sub_buffer.buf.put_slice(self.buf.chunk());
        Ok(sub_buffer)
    }
}

impl Read for Buffer {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.buf.read(buf)
    }
}

impl Write for Buffer {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buf.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.buf.flush()
    }
}
