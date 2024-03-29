use crate::leb128::{encode_i32_to_leb128, encode_u32_to_leb128};

#[derive(Debug, PartialEq, Eq)]
pub struct Buffer {
    pub bytes: Vec<u8>,
}

impl Buffer {
    pub fn new() -> Self {
        Self { bytes: vec![] }
    }

    pub fn write_u8(&mut self, value: u8) {
        self.bytes.push(value);
    }

    pub fn write_bytes(&mut self, bytes: Vec<u8>) {
        self.bytes.extend(bytes);
    }

    pub fn write_u32(&mut self, mut value: u32) {
        let bytes = encode_u32_to_leb128(value);
        self.write_bytes(bytes);
    }

    pub fn write_i32(&mut self, mut value: i32) {
        let bytes = encode_i32_to_leb128(value);
        self.write_bytes(bytes);
    }

    pub fn write_string(&mut self, value: String) {
        let bytes = value.into_bytes();
        self.write_u32(bytes.len() as u32);
        self.write_bytes(bytes);
    }

    pub fn write_vector(&mut self, value: Vec<u8>) {
        self.write_u32(value.len() as u32);
        self.write_bytes(value);
    }

    pub fn write_to_file(&self, path: &str) {
        use std::fs::File;
        use std::io::Write;

        let mut file = File::create(path).unwrap();
        file.write_all(&self.bytes).unwrap();
    }

    pub fn clear(&mut self) {
        self.bytes.clear();
    }
}

#[cfg(test)]
mod leb128 {
    use crate::parser::Parser;

    use super::*;

    #[test]
    fn test_write_u32() {
        let mut buffer = Buffer::new();
        buffer.write_u32(0);
        assert_eq!(buffer.bytes, vec![0x00]);
        let mut value: u32;
        let mut size: u32;
        (value, size) = Parser::read_u32(&mut buffer.bytes).expect("Invalid u32");
        assert_eq!(value, 0);
        assert_eq!(size, 1);
        buffer.clear();

        buffer.write_u32(1);
        assert_eq!(buffer.bytes, vec![0x01]);
        (value, size) = Parser::read_u32(&mut buffer.bytes).expect("Invalid u32");
        assert_eq!(value, 1);
        assert_eq!(size, 1);
        buffer.clear();

        buffer.write_u32(127);
        assert_eq!(buffer.bytes, vec![0x7F]);
        (value, size) = Parser::read_u32(&mut buffer.bytes).expect("Invalid u32");
        assert_eq!(value, 127);
        assert_eq!(size, 1);
        buffer.clear();

        buffer.write_u32(128);
        assert_eq!(buffer.bytes, vec![0x80, 0x01]);
        (value, size) = Parser::read_u32(&mut buffer.bytes).expect("Invalid u32");
        assert_eq!(value, 128);
        assert_eq!(size, 2);
        buffer.clear();

        buffer.write_u32(129);
        assert_eq!(buffer.bytes, vec![0x81, 0x01]);
        (value, size) = Parser::read_u32(&mut buffer.bytes).expect("Invalid u32");
        assert_eq!(value, 129);
        assert_eq!(size, 2);
        buffer.clear();

        buffer.write_u32(130);
        assert_eq!(buffer.bytes, vec![0x82, 0x01]);
        (value, size) = Parser::read_u32(&mut buffer.bytes).expect("Invalid u32");
        assert_eq!(value, 130);
        assert_eq!(size, 2);
        buffer.clear();

        buffer.write_u32(255);
        assert_eq!(buffer.bytes, vec![0xFF, 0x01]);
        (value, size) = Parser::read_u32(&mut buffer.bytes).expect("Invalid u32");
        assert_eq!(value, 255);
        assert_eq!(size, 2);
        buffer.clear();

        buffer.write_u32(256);
        assert_eq!(buffer.bytes, vec![0x80, 0x02]);
        (value, size) = Parser::read_u32(&mut buffer.bytes).expect("Invalid u32");
        assert_eq!(value, 256);
        assert_eq!(size, 2);
        buffer.clear();

        buffer.write_u32(1023);
        assert_eq!(buffer.bytes, vec![0xFF, 0x07]);
        (value, size) = Parser::read_u32(&mut buffer.bytes).expect("Invalid u32");
        assert_eq!(value, 1023);
        assert_eq!(size, 2);
        buffer.clear();

        buffer.write_u32(1024);
        assert_eq!(buffer.bytes, vec![0x80, 0x08]);
        (value, size) = Parser::read_u32(&mut buffer.bytes).expect("Invalid u32");
        assert_eq!(value, 1024);
        assert_eq!(size, 2);
        buffer.clear();

        buffer.write_u32(65535);
        assert_eq!(buffer.bytes, vec![0xFF, 0xFF, 0x03]);
        (value, size) = Parser::read_u32(&mut buffer.bytes).expect("Invalid u32");
        assert_eq!(value, 65535);
        assert_eq!(size, 3);
        buffer.clear();

        buffer.write_u32(65536);
        assert_eq!(buffer.bytes, vec![0x80, 0x80, 0x04]);
        (value, size) = Parser::read_u32(&mut buffer.bytes).expect("Invalid u32");
        assert_eq!(value, 65536);
        assert_eq!(size, 3);
        buffer.clear();
    }

    #[test]
    fn test_write_i32() {
        let mut buffer = Buffer::new();
        buffer.write_i32(0);
        assert_eq!(buffer.bytes, vec![0x00]);
        let mut i_value: i32;
        let mut size: u32;

        (i_value, size) = Parser::read_i32(&mut buffer.bytes).expect("Invalid u32");
        assert_eq!(i_value, 0);
        assert_eq!(size, 1);
        buffer.clear();

        buffer.write_i32(-1);
        assert_eq!(buffer.bytes, vec![0x7F]);
        (i_value, size) = Parser::read_i32(&mut buffer.bytes).expect("Invalid u32");
        assert_eq!(i_value, -1);
        assert_eq!(size, 1);
        buffer.clear();

        buffer.write_i32(-64);
        assert_eq!(buffer.bytes, vec![0x40]);
        (i_value, size) = Parser::read_i32(&mut buffer.bytes).expect("Invalid u32");
        assert_eq!(i_value, -64);
        assert_eq!(size, 1);
        buffer.clear();

        buffer.write_i32(-128);
        assert_eq!(buffer.bytes, vec![0x80, 0x7F]);
        (i_value, size) = Parser::read_i32(&mut buffer.bytes).expect("Invalid u32");
        assert_eq!(i_value, -128);
        assert_eq!(size, 2);
        buffer.clear();
    }
}
