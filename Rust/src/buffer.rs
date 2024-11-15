use std::{
    collections::HashMap,
    io::{Read, Write},
};

use algorithm::buf::{BinaryMut, Bt, BtMut};

use crate::{encode::{encode_str_raw, encode_varint}, error::HpResult, ErrorKind, HpError, Value};

#[derive(Debug)]
pub struct Buffer<T: Bt + BtMut = BinaryMut> {
    pub buf: T,
    pub str_arr: Vec<String>,
    pub str_map: HashMap<String, u16>,
}

impl Buffer<BinaryMut> {
    pub fn new() -> Buffer<BinaryMut> {
        Buffer {
            buf: BinaryMut::new(),
            str_arr: Vec::new(),
            str_map: HashMap::new(),
        }
    }
}

impl<T: Bt + BtMut> Buffer<T> {
    pub fn new_with(buf: T) -> Self {
        Buffer {
            buf,
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

    pub fn len(&self) -> usize {
        self.buf.chunk().len()
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

impl<B: Bt+BtMut> Bt for Buffer<B> {
    fn remaining(&self) -> usize {
        self.buf.remaining()
    }

    fn chunk(&self) -> &[u8] {
        self.buf.chunk()
    }

    fn advance(&mut self, n: usize) {
        self.buf.advance(n)
    }

    fn advance_chunk(&mut self, n: usize) -> &[u8] {
        self.buf.advance_chunk(n)
    }

    fn into_binary(self) -> algorithm::buf::Binary {
        self.buf.into_binary()
    }
}


unsafe impl<B: Bt+BtMut> BtMut for Buffer<B> {
    fn remaining_mut(&self) -> usize {
        self.buf.remaining_mut()
    }

    unsafe fn advance_mut(&mut self, cnt: usize) {
        self.buf.advance_mut(cnt)
    }

    fn chunk_mut(&mut self) -> &mut [std::mem::MaybeUninit<u8>] {
        self.buf.chunk_mut()
    }
}