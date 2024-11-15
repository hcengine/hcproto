
mod error;
mod ser;
mod values;
mod buffer;
pub mod encode;
pub mod decode;
mod de;


pub use buffer::Buffer;
pub use values::*;
pub use error::{HpError, ErrorKind, HpResult, make_extension_error};
pub use ser::to_buffer;
pub use de::from_buffer;

pub use encode::*;
pub use decode::*;

#[cfg(test)]
mod tests {
    use crate::{decode_varint, encode_varint, Buffer, Value};


    #[test]
    fn test_varint() {
        let mut buffer = Buffer::new();
        encode_varint(&mut buffer, &Value::U16(3)).unwrap();
        let ret = decode_varint(&mut buffer).unwrap();
        assert_eq!(ret, Value::U16(3));
    }
    
}