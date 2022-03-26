use crate::module::{
    function::{Block, Function},
    function_type::FunctionType,
    number::{Number, NumberType},
    opcode::OpCode,
    section::{ExportDesc, SectionId, TypeSection},
    Module,
};
use crate::util::byte::byte2string;
use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{BufReader, Read},
};

const UNSIGNED_LEB128_MAX_BITS: usize = 32;

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
                    let [section_size, _] = self.read_unsigned_leb128();
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
                            self.decode_export_section(&mut module).unwrap()
                        }
                        SectionId::CodeSectionId => self.decode_code_section(&mut module),
                    }
                }
                Err(_) => break,
            }
        }

        for func in module.functions {
            println!("{}", func.inspect());
        }

        for key in module.exported.keys() {
            println!("{}: {}", key, module.exported.get(key).unwrap().inspect());
        }

        Ok(())
    }

    fn decode_type_section(&mut self, module: &mut Module) {
        println!("Decode Type Section");

        let [signature_count, _] = self.read_unsigned_leb128();
        println!("Signature count: {}", signature_count);

        let mut byte_buf = [0; 1];
        self.reader.read_exact(&mut byte_buf).unwrap();
        TypeSection::validate_header(byte_buf[0]);

        for s_i in 0..signature_count {
            println!("Signature {}", s_i + 1);

            let mut func_type = FunctionType::default();

            let [parameter_count, _] = self.read_unsigned_leb128();
            for p_i in 0..parameter_count {
                let value = self.decode_type().unwrap();
                println!("Parameter {} Type {:?}", p_i + 1, value);
                func_type.parameters.push(value);
            }

            let [result_count, _] = self.read_unsigned_leb128();

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

        let [function_count, _] = self.read_unsigned_leb128();
        println!("Function count: {}", function_count);
        for i in 0..function_count {
            self.read_unsigned_leb128();
            let func_type = module.function_types[i].clone();
            module.functions.push(Function::new(func_type))
        }
    }

    fn decode_export_section(&mut self, module: &mut Module) -> Result<(), Box<dyn Error>> {
        println!("Decode Export Section");

        let [export_count, _] = self.read_unsigned_leb128();
        println!("Export count: {}", export_count);
        for _ in 0..export_count {
            let [name_size, _] = self.read_unsigned_leb128();
            let mut name_buf = vec![0; name_size];
            self.reader.read_exact(&mut name_buf)?;
            let name = std::str::from_utf8(&name_buf).unwrap();

            let mut desc_buf = [0; 1];
            self.reader.read_exact(&mut desc_buf)?;

            let mut index_buf = [0; 1];
            self.reader.read_exact(&mut index_buf)?;

            match ExportDesc::from_usize(desc_buf[0]).unwrap() {
                ExportDesc::Func => {
                    if module.exported.contains_key(name) {
                        panic!("{} key already exists", name);
                    }
                    module.exported.insert(
                        name.to_string(),
                        module.functions[usize::from(index_buf[0])].clone(),
                    );
                }
                ExportDesc::Table => todo!(),
                ExportDesc::LinearMemory => todo!(),
                ExportDesc::GlobalVariable => todo!(),
            }
        }

        Ok(())
    }

    fn decode_code_section(&mut self, module: &mut Module) {
        println!("Decode Code Section");

        let [func_body_count, _] = self.read_unsigned_leb128();
        println!("func_body Count: {}", func_body_count);

        for func_idx in 0..func_body_count {
            println!("# func_body {}", func_idx);

            let [func_body_size, _] = self.read_unsigned_leb128();
            println!("# func_body size: {}", func_body_size);

            let mut local_var_byte_size: usize = 0;
            let [local_var_count, local_var_count_byte_size] = self.read_unsigned_leb128();
            local_var_byte_size += local_var_count_byte_size;
            println!("# Local Var Count: {}", local_var_count);

            for _ in 0..local_var_count {
                let [local_var_type_count, local_var_type_count_byte_size] =
                    self.read_unsigned_leb128();
                local_var_byte_size += local_var_type_count_byte_size;
                let local_var_type = self.decode_type().unwrap();
                local_var_byte_size += 1;
                println!(
                    "Local Var Type: {} Count: {:x}",
                    local_var_type.inspect(),
                    local_var_type_count
                );

                module.functions[func_idx].local_vars = vec![local_var_type; local_var_type_count];
            }
            let mut expression_buf: Vec<u8> = vec![0; func_body_size - local_var_byte_size];
            self.reader.read_exact(&mut expression_buf).unwrap();
            module.functions[func_idx].expressions = expression_buf;

            println!("{}", module.functions[func_idx].inspect());

            let mut expressions = module.functions[func_idx].expressions.clone();
            let mut blocks: HashMap<usize, Block> = HashMap::new();

            let mut block_stack = vec![Block::new(expressions.clone(), 0)];

            expressions.reverse();
            loop {
                if expressions.len() == 0 {
                    break;
                }
                match self.read_next_structured_instruction(&mut expressions) {
                    Some(structured_instruction) => {
                        let idx = func_body_size - expressions.len() - 2;
                        match OpCode::from_byte(structured_instruction) {
                            OpCode::End => {
                                let mut block = block_stack.pop().unwrap();
                                block.end_idx = idx;
                                block.instructions = module.functions[func_idx].expressions
                                    [block.start_idx..block.end_idx]
                                    .to_vec();
                                blocks.insert(block.start_idx, block);
                            }
                            _ => {
                                println!("SI byte: {:x} idx: {}", structured_instruction, idx);

                                let block = Block::new(vec![], idx);
                                block_stack.push(block);
                            }
                        };
                    }
                    None => {}
                };
            }
        }

        // println!("Expression");
        // for i in 0..module.functions[0].expressions.len() {
        //     println!(
        //         "{}: {}, {:x}, {:b}",
        //         i,
        //         module.functions[0].expressions[i],
        //         module.functions[0].expressions[i],
        //         module.functions[0].expressions[i]
        //     );
        // }

        println!("Rest binary");
        for i in 0..100 {
            let mut buf = [0; 1];
            self.reader.read_exact(&mut buf).unwrap();
            println!("{}: {}, {:x}, {:b}", i, buf[0], buf[0], buf[0]);
        }
    }

    fn decode_type(&mut self) -> Result<Number, Box<dyn Error>> {
        let mut buf = [0; 1];
        self.reader.read_exact(&mut buf)?;
        NumberType::decode_type(buf[0])
    }

    fn read_next_structured_instruction(&mut self, expression: &mut Vec<u8>) -> Option<u8> {
        let mut byte;
        loop {
            if expression.len() == 0 {
                return None;
            }
            byte = expression.pop().unwrap();
            match OpCode::from_byte(byte) {
                OpCode::Block | OpCode::Loop | OpCode::If | OpCode::End => {
                    // println!("block or loop or if or end");
                    break;
                }
                OpCode::Br | OpCode::BrIf => {
                    // println!("br or br_if");
                    Decoder::decode_unsigned_leb128(expression);
                }
                OpCode::GetLocal
                | OpCode::SetLocal
                | OpCode::TeeLocal
                | OpCode::GetGlobal
                | OpCode::SetGlobal => {
                    // println!("get/set local/global");
                    Decoder::decode_unsigned_leb128(expression);
                }
                OpCode::I32Const | OpCode::I64Const | OpCode::F32Const | OpCode::F64Const => {
                    // println!("constants");
                    Decoder::decode_signed_leb128(expression);
                }
                _ => {}
            };
        }

        Some(byte)
    }

    fn discard_section(&mut self, size: usize) -> Result<(), Box<dyn Error>> {
        let mut buf = vec![0; size];
        let result = self.reader.read_exact(&mut buf)?;
        for b in buf.clone() {
            println!("section byte: {}", b);
        }
        Ok(result)
    }

    fn read_unsigned_leb128(&mut self) -> [usize; 2] {
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
            if shift > UNSIGNED_LEB128_MAX_BITS {
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
            if shift > UNSIGNED_LEB128_MAX_BITS {
                panic!("Invalid LEB128 encoding");
            }
        }
        if (value >> (shift - 1)) & 1 == 1 {
            value |= !0 << shift;
        }
        [value, byte_count]
    }

    fn decode_unsigned_leb128(buf: &mut Vec<u8>) -> [usize; 2] {
        let mut value: usize = 0;
        let mut shift: usize = 0;
        let mut byte_count: usize = 0;

        loop {
            let byte = buf.pop().unwrap();
            value |= ((byte & 0x7F) as usize) << shift;
            shift += 7;

            if ((byte >> 7) & 1) != 1 {
                break;
            }
            byte_count += 1;
            if shift > UNSIGNED_LEB128_MAX_BITS {
                panic!("Invalid LEB128 encoding");
            }
        }
        [value, byte_count]
    }

    fn decode_signed_leb128(buf: &mut Vec<u8>) -> [usize; 2] {
        let mut value: usize = 0;
        let mut shift: usize = 0;
        let mut byte_count: usize = 0;

        loop {
            let byte = buf.pop().unwrap();
            value |= ((byte & 0x7F) as usize) << shift;
            shift += 7;

            if ((byte >> 7) & 1) != 1 {
                break;
            }
            byte_count += 1;
            if shift > UNSIGNED_LEB128_MAX_BITS {
                panic!("Invalid LEB128 encoding");
            }
        }
        if (value >> (shift - 1)) & 1 == 1 {
            value |= !0 << shift;
        }

        [value, byte_count]
    }
}
