use crate::module::{
    function::Function,
    section::{SectionId, TypeSection},
    value::{Value, ValueType},
    Module,
};
use crate::util::byte::byte2string;
use std::{
    error::Error,
    fs::File,
    io::{BufReader, Read},
};

pub struct Decoder {
    reader: BufReader<File>,
}

impl Decoder {
    pub fn new(path: &str) -> Result<Decoder, Box<dyn Error>> {
        Ok(Decoder {
            reader: BufReader::new(File::open(path)?),
        })
    }

    pub fn validate_header(&mut self) {
        let mut header_buf = [0; 8];
        self.reader.read_exact(&mut header_buf).unwrap();
        let header = byte2string(Box::new(header_buf));
        let module = Module::default();
        if !module.valid_header(&header) {
            panic!("Invalid wasm header: {}", header);
        }
    }

    pub fn decode_section(&mut self) -> Result<(), Box<dyn Error>> {
        let mut byte_buf = [0; 1];
        self.reader.read_exact(&mut byte_buf).unwrap();
        let section_id = byte_buf[0];
        let section_size = self.read_unsigned_leb128();
        println!("Section ID: {} Size: {}", section_id, section_size);

        match SectionId::from_usize(section_id).unwrap() {
            SectionId::CustomSectionId => {
                println!("Custom Section");
            }
            SectionId::TypeSectionId => {
                println!("Type Section");

                let signature_count = self.read_unsigned_leb128();
                println!("Signature count: {}", signature_count);

                self.reader.read_exact(&mut byte_buf).unwrap();
                TypeSection::validate_header(byte_buf[0]);

                for s_i in 0..signature_count {
                    println!("Signature {}", s_i + 1);

                    let mut func = Function::default();

                    let parameter_count = self.read_unsigned_leb128();
                    for p_i in 0..parameter_count {
                        let value = self.decode_type().unwrap();
                        println!("Parameter {} Type {:?}", p_i + 1, value);
                        func.parameters.push(value);
                    }

                    let result_count = self.read_unsigned_leb128();
                    for r_i in 0..result_count {
                        let value = self.decode_type().unwrap();
                        println!("Result {} Type {:?}", r_i + 1, value);
                        func.results.push(value);
                    }
                }
            }
            SectionId::FunctionSectionId => {
                println!("Function Section");
            }
            SectionId::ExportSectionId => {
                println!("Export Section");
            }
            SectionId::CodeSectionId => {
                println!("Code Section");
            }
        }

        let size = self.read_unsigned_leb128();
        match self.discard_section(size) {
            Ok(()) => println!("Discard section"),
            Err(err) => panic!("Failed to discard section {}", err),
        }

        Ok(())
    }

    fn decode_type(&mut self) -> Result<Value, Box<dyn Error>> {
        let mut buf = [0; 1];
        self.reader.read_exact(&mut buf)?;
        Ok(match ValueType::from_byte(buf[0]).unwrap() {
            ValueType::Int32 => Value::i32(),
            ValueType::Int64 => Value::i64(),
            ValueType::Float32 => Value::f32(),
            ValueType::Float64 => Value::f64(),
        })
    }

    fn discard_section(&mut self, size: usize) -> Result<(), Box<dyn Error>> {
        let mut buf = vec![0; size];
        let result = self.reader.read_exact(&mut buf)?;
        for b in buf.clone() {
            println!("section byte: {}", b);
        }
        Ok(result)
    }

    fn read_unsigned_leb128(&mut self) -> usize {
        let mut value: usize = 0;
        let mut shift: u32 = 0;
        let mut buf: [u8; 1] = [0; 1];
        let unsigned_leb128_max_bits = 32;

        loop {
            match self.reader.read_exact(&mut buf) {
                Ok(()) => {
                    value |= ((buf[0] & 0x7F) as usize) << shift;
                    shift += 7;

                    if ((buf[0] >> 7) & 1) != 1 {
                        break;
                    }
                    if shift > unsigned_leb128_max_bits {
                        panic!("Invalid LEB128 encoding");
                    }
                }
                Err(err) => panic!("Failed to read buffer {}", err),
            }
        }
        value
    }
}
