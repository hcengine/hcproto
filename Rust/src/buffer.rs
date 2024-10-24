use std::{
    collections::HashMap,
    io::{Read, Write},
};

use algorithm::buf::BinaryMut;

use crate::error::HpResult;

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
        // if idx as usize >= self.str_arr.len() {
        //     fail!((ErrorKind::BufferOverMaxError, "must left space to read "));
        // } else {
        Ok(self.str_arr[idx as usize].clone())
        // }
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
