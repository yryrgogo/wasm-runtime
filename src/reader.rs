use std::{
    error::Error,
    fs::File,
    io::{BufReader, Read},
};

pub const LEB128_MAX_BITS: usize = 32;

pub struct WasmBinaryReader {
    pub buffer: Vec<u8>,
    pc: usize,
}

impl WasmBinaryReader {
    pub fn new(path: Option<&str>, wasm_module: Option<Vec<u8>>) -> Result<Self, Box<dyn Error>> {
        let mut buffer = Vec::new();

        if let Some(p) = path {
            let mut reader = BufReader::new(File::open(p)?);
            let mut buffer = Vec::new();
            reader.read_to_end(&mut buffer)?;
        } else {
            if let Some(module) = wasm_module {
                buffer = module;
            } else {
                panic!("Wasm モジュールを渡してください。")
            }
        }

        Ok(WasmBinaryReader {
            buffer: buffer,
            pc: 0,
        })
    }

    pub fn read_header(&mut self) -> Vec<u8> {
        self.pc += 8;
        self.buffer[self.pc - 8..self.pc].to_vec()
    }

    pub fn read_next_byte(&mut self) -> Option<u8> {
        self.pc += 1;
        if let Some(byte) = self.buffer.get(self.pc - 1) {
            Some(*byte)
        } else {
            None
        }
    }

    pub fn read_bytes(&mut self, size: usize) -> Vec<u8> {
        self.pc += size;
        self.buffer[self.pc - size..self.pc].to_vec()
    }

    pub fn read_unsigned_leb128(&mut self) -> [usize; 2] {
        let mut value: usize = 0;
        let mut shift: usize = 0;
        let mut byte_count: usize = 0;

        loop {
            self.pc += 1;
            let first_byte = self.buffer[self.pc - 1..self.pc][0];
            value |= ((first_byte & 0x7F) as usize) << shift;
            shift += 7;
            byte_count += 1;

            if ((first_byte >> 7) & 1) != 1 {
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
        let mut byte_count: usize = 0;

        loop {
            let first_byte = self.buffer[self.pc - 1..self.pc][0];
            value |= ((first_byte & 0x7F) as usize) << shift;
            shift += 7;
            byte_count += 1;

            if ((first_byte >> 7) & 1) != 1 {
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
