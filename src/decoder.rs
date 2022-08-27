use super::module::Module;
use std::error::Error;

pub const LEB128_MAX_BITS: usize = 32;

pub struct Decoder {}

impl Decoder {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {})
    }

    pub fn decode(&self, bytes: &mut Vec<u8>) -> Result<Module, Box<dyn Error>> {
        let (magic, version) = self.decode_header(bytes).expect("Invalid header");
        Module::validate_magic(&magic);
        Module::validate_version(&version);

        let module = Module::new().expect("Invalid module");

        if bytes.len() == 0 {
            return Ok(module);
        }

        Decoder::read_u32(bytes);

        Ok(module)
    }

    fn decode_header(&self, bytes: &mut Vec<u8>) -> Result<(Vec<u8>, Vec<u8>), Box<dyn Error>> {
        let magic_bytes = bytes[0..4].to_vec();
        let version = bytes[4..8].to_vec();
        *bytes = bytes[8..].to_vec();
        Ok((magic_bytes, version))
    }

    pub fn read_u32(bytes: &mut Vec<u8>) -> (usize, usize) {
        let mut value: usize = 0;
        let mut shift: usize = 0;
        let mut byte_count: usize = 0;

        loop {
            let byte = bytes[0];
            (*bytes).drain(0..1);
            value |= usize::from(byte & 0x7f) << shift;
            shift += 7;
            byte_count += 1;

            if ((byte >> 7) & 1) != 1 {
                break;
            }
            if shift > LEB128_MAX_BITS {
                panic!("unsigned LEB128 overflow");
            }
        }
        (value, byte_count)
    }

    fn read_i32(bytes: &mut Vec<u8>) -> (isize, usize) {
        let mut value: isize = 0;
        let mut shift: usize = 0;
        let mut byte_count: usize = 0;

        loop {
            let byte = bytes[0];
            (*bytes).drain(0..1);
            value |= isize::from(byte & 0x7F) << shift;
            shift += 7;
            byte_count += 1;

            if ((byte >> 7) & 1) != 1 {
                break;
            }
            if shift > LEB128_MAX_BITS {
                panic!("signed LEB128 overflow");
            }
        }
        if (value >> (shift - 1)) & 1 == 1 {
            value |= !0 << shift;
        }
        (value, byte_count)
    }
}

#[cfg(test)]
mod leb128_tests {
    use super::*;

    #[test]
    fn read_u32_case1() {
        let mut bytes = vec![229, 142, 38, 0, 0, 0, 0, 0];
        let (value, size) = Decoder::read_u32(&mut bytes);
        assert_eq!(value, 624485);
        assert_eq!(size, 3);
    }

    #[test]
    fn read_u32_case2() {
        let mut bytes = vec![0x80, 0x80, 0xC0, 0x00, 0x0B];
        let (value, size) = Decoder::read_u32(&mut bytes);

        assert_eq!(value, 1048576);
        assert_eq!(size, 4);
    }

    #[test]
    fn test_read_i32() {
        let mut bytes = vec![127, 0, 0, 0, 0, 0, 0, 0];
        let (value, size) = Decoder::read_i32(&mut bytes);

        assert_eq!(value, -1);
        assert_eq!(size, 1);
    }
}
