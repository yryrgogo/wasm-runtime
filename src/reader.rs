use std::{
    error::Error,
    fs::File,
    io::{BufReader, Read},
};

pub const LEB128_MAX_BITS: usize = 32;

pub struct WasmModuleReader {
    reader: BufReader<File>,
}

impl WasmModuleReader {
    pub fn new(path: &str) -> Result<Self, Box<dyn Error>> {
        Ok(WasmModuleReader {
            reader: BufReader::new(File::open(path)?),
        })
    }

    pub fn read_header(&mut self) -> [u8; 8] {
        let mut buf = [0; 8];
        self.reader
            .read_exact(&mut buf)
            .unwrap_or_else(|_| panic!("ヘッダの読み取りに失敗しました。"));
        buf
    }

    pub fn read_next_byte(&mut self) -> u8 {
        let mut buf = [0; 1];
        self.reader
            .read_exact(&mut buf)
            .unwrap_or_else(|_| panic!("1byte の読み取りに失敗しました。"));
        buf[0]
    }

    pub fn read_bytes(&mut self, size: usize) -> Vec<u8> {
        let mut buf = vec![0; size];
        self.reader
            .read_exact(&mut buf)
            .unwrap_or_else(|_| panic!("{} byte の読み取りに失敗しました。", size));
        buf
    }

    pub fn read_unsigned_leb128(&mut self) -> [usize; 2] {
        let mut value: usize = 0;
        let mut shift: usize = 0;
        let mut buf: [u8; 1] = [0; 1];
        let mut byte_count: usize = 0;

        loop {
            self.reader.read_exact(&mut buf).unwrap();
            value |= ((buf[0] & 0x7F) as usize) << shift;
            shift += 7;
            byte_count += 1;

            if ((buf[0] >> 7) & 1) != 1 {
                break;
            }
            if shift > LEB128_MAX_BITS {
                panic!("Invalid LEB128 encoding");
            }
        }
        [value, byte_count]
    }

    fn read_signed_leb128(&mut self) -> [usize; 2] {
        let mut value: usize = 0;
        let mut shift: usize = 0;
        let mut buf: [u8; 1] = [0; 1];
        let mut byte_count: usize = 0;

        loop {
            self.reader.read_exact(&mut buf).unwrap();
            value |= ((buf[0] & 0x7F) as usize) << shift;
            shift += 7;
            byte_count += 1;

            if ((buf[0] >> 7) & 1) != 1 {
                break;
            }
            if shift > LEB128_MAX_BITS {
                panic!("Invalid LEB128 encoding");
            }
        }
        if (value >> (shift - 1)) & 1 == 1 {
            value |= !0 << shift;
        }
        [value, byte_count]
    }
}
