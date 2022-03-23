use crate::module::{
    function::Function,
    function_type::FunctionType,
    number::{Number, NumberType},
    section::{SectionId, TypeSection},
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
        let mut module = Module::default();
        let mut byte_buf = [0; 1];

        loop {
            match self.reader.read_exact(&mut byte_buf) {
                Ok(()) => {
                    let section_id = byte_buf[0];
                    let section_size = self.read_unsigned_leb128();
                    println!("Section ID: {} Size: {}", section_id, section_size);

                    match SectionId::from_usize(section_id).unwrap() {
                        SectionId::CustomSectionId => {
                            println!("Custom Section");
                            match self.discard_section(section_size) {
                                Ok(()) => println!("Discard section"),
                                Err(err) => panic!("Failed to discard section {}", err),
                            }
                        }
                        SectionId::TypeSectionId => self.decode_type_section(&mut module),
                        SectionId::FunctionSectionId => self.decode_function_section(&mut module),
                        SectionId::ExportSectionId => {
                            println!("Export Section");
                            match self.discard_section(section_size) {
                                Ok(()) => println!("Discard section"),
                                Err(err) => panic!("Failed to discard section {}", err),
                            }
                        }
                        SectionId::CodeSectionId => {
                            println!("Code Section");
                            match self.discard_section(section_size) {
                                Ok(()) => println!("Discard section"),
                                Err(err) => panic!("Failed to discard section {}", err),
                            }
                        }
                    }
                }
                Err(_) => break,
            }
        }

        for func in module.functions {
            println!("{}", func.inspect());
        }

        Ok(())
    }

    fn decode_type_section(&mut self, module: &mut Module) {
        println!("Decode Type Section");

        let signature_count = self.read_unsigned_leb128();
        println!("Signature count: {}", signature_count);

        let mut byte_buf = [0; 1];
        self.reader.read_exact(&mut byte_buf).unwrap();
        TypeSection::validate_header(byte_buf[0]);

        for s_i in 0..signature_count {
            println!("Signature {}", s_i + 1);

            let mut func_type = FunctionType::default();

            let parameter_count = self.read_unsigned_leb128();
            for p_i in 0..parameter_count {
                let value = self.decode_type().unwrap();
                println!("Parameter {} Type {:?}", p_i + 1, value);
                func_type.parameters.push(value);
            }

            let result_count = self.read_unsigned_leb128();

            // NOTE: 202203時点の仕様では戻り値は1つまで
            assert_eq!(result_count, 1);

            for r_i in 0..result_count {
                let value = self.decode_type().unwrap();
                println!("Result {} Type {:?}", r_i + 1, value);
                func_type.results.push(value);
            }
            module.function_types.push(func_type);
        }
    }

    fn decode_function_section(&mut self, module: &mut Module) {
        println!("Decode Function Section");

        let function_count = self.read_unsigned_leb128();
        println!("Function count: {}", function_count);
        for i in 0..function_count {
            self.read_unsigned_leb128();
            let func_type = module.function_types[i].clone();
            module.functions.push(Function::new(func_type))
        }
    }

    fn decode_type(&mut self) -> Result<Number, Box<dyn Error>> {
        let mut buf = [0; 1];
        self.reader.read_exact(&mut buf)?;
        Ok(match NumberType::from_byte(buf[0]).unwrap() {
            NumberType::Int32 => Number::i32(),
            NumberType::Int64 => Number::i64(),
            NumberType::Float32 => Number::f32(),
            NumberType::Float64 => Number::f64(),
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
