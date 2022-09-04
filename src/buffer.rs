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
        // unsigned leb128
        value |= 0;
        let mut result: Vec<u8> = vec![];
        loop {
            let byte = value & 0b01111111;
            value >>= 7;
            if value == 0 {
                result.push(byte as u8);
                break;
            } else {
                result.push((byte | 0b10000000) as u8);
            }
        }
        self.write_bytes(result);
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
}
