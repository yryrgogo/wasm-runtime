use crate::reader::{WasmBinaryReader, LEB128_MAX_BITS};
use crate::{
    export::ExportMap,
    module::{
        function::{Block, Function},
        function_type::FunctionType,
        number::NumberType,
        opcode::OpCode,
        section::{ExportDesc, SectionId, TypeSection},
        Module,
    },
};
use std::{collections::HashMap, error::Error};

pub struct Decoder {
    pub module: Module,
    pub reader: WasmBinaryReader,
}

impl Decoder {
    pub fn new(
        path: Option<&str>,
        wasm_module: Option<Vec<u8>>,
    ) -> Result<Decoder, Box<dyn Error>> {
        Ok(Decoder {
            module: Module::default(),
            reader: WasmBinaryReader::new(path, wasm_module)?,
        })
    }

    pub fn run(&mut self) {
        self.decode_header();
        self.decode_section();
    }

    pub fn decode_header(&mut self) {
        let header = String::from_utf8(Vec::from(self.reader.read_header()))
            .unwrap_or_else(|_| panic!("ヘッダの u8 -> String 変換に失敗しました。"));
        if !self.module.valid_header(&header) {
            panic!("Invalid wasm header: {}", header);
        }
    }

    pub fn decode_section(&mut self) {
        loop {
            match self.decode_section_id() {
                Some(section_id) => self.decode_section_body(section_id).unwrap_or_else(|err| {
                    panic!("Section Body のでコードに失敗しました。 {:?}", err)
                }),
                None => break,
            }
        }
    }

    pub fn decode_section_id(&mut self) -> Option<u8> {
        self.reader.read_next_byte()
    }

    pub fn decode_section_body(&mut self, section_id: u8) -> Result<(), Box<dyn Error>> {
        let [section_size, decoded_size] = self.reader.read_unsigned_leb128();
        println!(
            "Section ID: {} Size: {} Decoded Size: {}",
            section_id, section_size, decoded_size
        );

        match SectionId::from_usize(section_id).unwrap() {
            SectionId::CustomSectionId => {
                println!("Custom Section");
                match self.discard_section(section_size) {
                    Ok(()) => println!("Discard section"),
                    Err(err) => panic!("Failed to discard section {}", err),
                }
            }
            SectionId::TypeSectionId => self.decode_type_section(),
            SectionId::FunctionSectionId => self.decode_function_section(),
            SectionId::ExportSectionId => self.decode_export_section(),
            SectionId::CodeSectionId => self.decode_code_section(),
        }

        for func in self.module.functions.iter_mut() {
            println!("{}", func.inspect());
        }

        for key in self.module.exported.keys() {
            println!(
                "{}: {}",
                key,
                self.module.exported.get(key).unwrap().function.inspect()
            );
        }

        Ok(())
    }

    /// Type Section
    /// ```
    /// [
    ///  signature count,
    ///  type section header,
    ///  function parameter count,
    ///  function parameter type,
    ///  ...,
    ///  return value type
    /// ]
    /// ```
    fn decode_type_section(&mut self) {
        println!("#[Decode Type Section]");

        let [signature_count, size] = self.reader.read_unsigned_leb128();
        println!(
            "  Signature count: {} Decoded size: {}",
            signature_count, size
        );

        TypeSection::validate_header(
            self.reader
                .read_next_byte()
                .unwrap_or_else(|| panic!("  TypeSection のヘッダが見つかりません。")),
        );

        for s_i in 0..signature_count {
            println!("  Signature {}", s_i + 1);

            let mut func_type = FunctionType::default();

            let [parameter_count, _] = self.reader.read_unsigned_leb128();
            for p_i in 0..parameter_count {
                let num_type = self.decode_type().unwrap();
                println!("  Parameter {} Type {:?}", p_i + 1, num_type);
                func_type.parameters.push(num_type);
            }

            let [result_count, _] = self.reader.read_unsigned_leb128();

            // NOTE: 202203時点の仕様では戻り値は1つまで
            assert_eq!(result_count, 1);

            for r_i in 0..result_count {
                let value = self.decode_type().unwrap();
                println!("  Result {} Type {:?}", r_i + 1, value);
                func_type.results.push(value);
            }
            self.module.function_types.push(func_type);
        }
    }

    /// Function Section
    /// ```
    /// [
    ///  function count,
    ///  function type index,
    /// ]
    /// ```
    fn decode_function_section(&mut self) {
        println!("#[Decode Function Section]");

        let [function_count, _] = self.reader.read_unsigned_leb128();
        println!("  Function count: {}", function_count);

        for i in 0..function_count {
            let [_, _] = self.reader.read_unsigned_leb128();
            let func_type = self.module.function_types[i].clone();
            self.module
                .functions
                .push(Function::new(func_type, Some(self.module.functions.len())))
        }
    }

    /// Export Section
    /// ```
    /// [
    ///  export count,
    ///  export name size,
    ///  export desc （Export される値のタイプ）,
    ///  export desc index （Export される値のインデックス。Function の場合は Function の index）
    /// ]
    /// ```
    fn decode_export_section(&mut self) {
        println!("Decode Export Section");

        let [export_count, _] = self.reader.read_unsigned_leb128();

        println!("Export count: {}", export_count);
        for _ in 0..export_count {
            let [name_size, _] = self.reader.read_unsigned_leb128();
            let name_buf = self.reader.read_bytes(name_size);
            let name = std::str::from_utf8(&name_buf).unwrap();
            let desc = self
                .reader
                .read_next_byte()
                .unwrap_or_else(|| panic!("Export Section の desc byte が見つかりません"));

            match ExportDesc::from_usize(desc).unwrap() {
                ExportDesc::Func => {
                    if self.module.exported.contains_key(name) {
                        panic!("{} key already exists", name);
                    }
                    let index = self
                        .reader
                        .read_next_byte()
                        .unwrap_or_else(|| panic!("Export Section の index byte が見つかりません"));
                    let func_idx = usize::from(index);
                    self.module.exported.insert(
                        name.to_string(),
                        ExportMap {
                            index: func_idx,
                            function: self.module.functions[func_idx].clone(),
                        },
                    );
                }
                ExportDesc::Table => todo!(),
                ExportDesc::LinearMemory => todo!(),
                ExportDesc::GlobalVariable => todo!(),
            }
        }
    }

    fn decode_code_section(&mut self) {
        println!("Decode Code Section");

        let [func_body_count, _] = self.reader.read_unsigned_leb128();
        println!("func_body Count: {}", func_body_count);

        for func_idx in 0..func_body_count {
            self.decode_code_section_body(func_idx)
        }
    }

    fn decode_code_section_body(&mut self, func_idx: usize) {
        println!("# func_body {}", func_idx);

        let [func_body_size, _] = self.reader.read_unsigned_leb128();
        println!("# func_body size: {}", func_body_size);

        let mut local_var_byte_size: usize = 0;
        let [local_var_count, size] = self.reader.read_unsigned_leb128();
        local_var_byte_size += size;
        println!("# Local Var Count: {}", local_var_count);

        for _ in 0..local_var_count {
            let local_var_type_count_byte_size = self.decode_code_section_body_local_var(func_idx);
            local_var_byte_size += local_var_type_count_byte_size;
            local_var_byte_size += 1;
        }
        let expression_buf = self.reader.read_bytes(func_body_size - local_var_byte_size);
        self.module.functions[func_idx].expressions = expression_buf.to_vec();

        println!("{}", self.module.functions[func_idx].inspect());

        self.decode_code_section_body_block(func_idx);
    }

    fn decode_code_section_body_local_var(&mut self, func_idx: usize) -> usize {
        let [local_var_type_count, local_var_type_count_byte_size] =
            self.reader.read_unsigned_leb128();
        let local_var_type = self.decode_type().unwrap();
        println!(
            "Local Var Type: {} Count: {:x}",
            local_var_type.inspect(),
            local_var_type_count
        );

        self.module.functions[func_idx].local_vars = vec![local_var_type; local_var_type_count];
        local_var_type_count_byte_size
    }

    fn decode_code_section_body_block(&mut self, func_idx: usize) {
        let mut expressions = self.module.functions[func_idx].expressions.clone();
        let mut blocks: HashMap<usize, Block> = HashMap::new();

        let mut block_stack = vec![Block::new(
            2,
            self.module.function_types[func_idx].results.clone(),
            0,
            None,
        )];

        expressions.reverse();
        loop {
            if expressions.len() == 0 {
                break;
            }
            match self.read_next_structured_instruction(&mut expressions) {
                Some(structured_instruction) => {
                    let idx = self.module.functions[func_idx].expressions.len() - expressions.len();
                    println!("idx:{}", idx);
                    match OpCode::from_byte(structured_instruction) {
                        OpCode::End => {
                            let mut block = block_stack.pop().unwrap();
                            block.end_idx = idx;
                            blocks.insert(block.start_idx, block);
                        }
                        op => {
                            let opcode = self.reader.read_next_byte().unwrap_or_else(|| {
                                panic!("Block Section の arity 読み込みに失敗しました。")
                            });
                            let arity: Vec<NumberType> = if opcode == 0x40 {
                                vec![]
                            } else {
                                let v = NumberType::from_byte(opcode).unwrap_or_else(|| {
                                    panic!("NumberType に渡した byte 値が不正です。")
                                });
                                vec![v]
                            };
                            println!("[Structured Instruction] {:?} arity: {:?}", op, arity);
                            let block = Block::new(structured_instruction, arity, idx, None);
                            block_stack.push(block);
                        }
                    };
                }
                None => {}
            };
        }

        self.module.functions[func_idx].blocks = blocks;
    }

    fn decode_type(&mut self) -> Result<NumberType, Box<dyn Error>> {
        let byte = self
            .reader
            .read_next_byte()
            .unwrap_or_else(|| panic!("Value Type の byte 読み込みに失敗しました。"));
        NumberType::decode_type(byte)
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
        let bytes = self.reader.read_bytes(size);
        for b in bytes.clone() {
            println!("section byte: {}", b);
        }
        Ok(())
    }

    fn decode_unsigned_leb128(buf: &[u8]) -> [usize; 2] {
        let mut value: usize = 0;
        let mut shift: usize = 0;
        let mut byte_count: usize = 0;

        loop {
            let byte = buf[0];
            byte_count += 1;
            value |= ((byte & 0x7F) as usize) << shift;
            shift += 7;

            if ((byte >> 7) & 1) != 1 {
                break;
            }
            if shift > LEB128_MAX_BITS {
                panic!("Invalid LEB128 encoding");
            }
        }
        [value, byte_count]
    }

    fn decode_signed_leb128(buf: &[u8]) -> [usize; 2] {
        let mut value: usize = 0;
        let mut shift: usize = 0;
        let mut byte_count: usize = 0;

        loop {
            let byte = buf[0];
            byte_count += 1;
            value |= ((byte & 0x7F) as usize) << shift;
            shift += 7;

            if ((byte >> 7) & 1) != 1 {
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

    pub fn inspect(&self) {
        for func in self.module.functions.clone() {
            println!("{}", func.inspect());
            for (_, block) in func.blocks {
                println!("{}", block.inspect());
            }
        }
    }
}

#[cfg(test)]
mod leb_tests {
    use super::*;

    #[test]
    fn can_read_unsigned_leb128() {
        let wasm_module = vec![229, 142, 38, 0, 0, 0, 0, 0];
        let mut decoder = Decoder::new(None, Some(wasm_module)).unwrap();
        let [value, size] = decoder.reader.read_unsigned_leb128();

        assert_eq!(value, 624485);
        assert_eq!(size, 3);
    }

    #[test]
    #[should_panic]
    fn cannot_read_unsigned_leb128() {
        let wasm_module = vec![];

        let mut decoder = Decoder::new(None, Some(wasm_module)).unwrap();
        decoder.reader.read_unsigned_leb128();
    }

    #[test]
    fn can_read_signed_leb128() {
        let wasm_module = vec![127, 0, 0, 0, 0, 0, 0, 0];
        let mut decoder = Decoder::new(None, Some(wasm_module)).unwrap();
        decoder.reader.read_signed_leb128();
    }

    #[test]
    #[should_panic]
    fn cannot_read_signed_leb128() {
        let wasm_module = vec![];
        let mut decoder = Decoder::new(None, Some(wasm_module)).unwrap();
        decoder.reader.read_signed_leb128();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_decode_header() {
        let wasm_module_header = vec![0x00, 0x61, 0x73, 0x6D, 0x01, 0x00, 0x00, 0x00];
        let mut decoder = Decoder::new(None, Some(wasm_module_header)).unwrap();

        assert_eq!(decoder.reader.buffer.len(), 8);

        decoder.decode_header();

        assert_eq!(decoder.reader.pc, 8);
    }

    #[test]
    #[should_panic]
    fn cannot_decode_header() {
        let wasm_module_header = vec![0x00, 0x61, 0x73, 0x6D, 0x01, 0x00, 0x00];
        let mut decoder = Decoder::new(None, Some(wasm_module_header)).unwrap();

        assert_eq!(decoder.reader.buffer.len(), 7);
        decoder.decode_header();
    }

    #[test]
    fn can_decode_type_section() {
        let wasm_module_type_section = vec![0x01, 0x60, 0x01, 0x7f, 0x01, 0x7f];
        let mut decoder = Decoder::new(None, Some(wasm_module_type_section)).unwrap();

        decoder.decode_type_section();

        let func_type = decoder.module.function_types[0].clone();

        assert_eq!(
            func_type.parameters,
            vec![NumberType::decode_type(0x7f).unwrap()]
        );
        assert_eq!(
            func_type.results,
            vec![NumberType::decode_type(0x7f).unwrap()]
        );
    }

    #[test]
    fn can_decode_function_section() {
        let wasm_module_type_section = vec![
            // Type Section
            0x01, 0x60, 0x01, 0x7f, 0x01, 0x7f, // Function Section
            0x01, 0x00,
        ];
        let mut decoder = Decoder::new(None, Some(wasm_module_type_section)).unwrap();

        // function_type を作るために、先に type section のデコードが必要
        decoder.decode_type_section();
        decoder.decode_function_section();

        for (i, function) in decoder.module.functions.iter().enumerate() {
            let func_type = decoder.module.function_types[i].clone();
            assert_eq!(function, &Function::new(func_type, Some(i)));
        }
    }

    #[test]
    fn can_decode_export_section() {
        let wasm_module_type_section = vec![
            // Type Section
            0x01, 0x60, 0x01, 0x7f, 0x01, 0x7f, // Function Section
            0x01, 0x00, // Export Section
            0x01, 0x03, 0x66, 0x69, 0x62, 0x00, 0x00,
        ];
        let mut decoder = Decoder::new(None, Some(wasm_module_type_section)).unwrap();

        // function_type を作るために、先に type section のデコードが必要
        decoder.decode_type_section();
        decoder.decode_function_section();
        decoder.decode_export_section();

        for (key, export_map) in decoder.module.exported {
            assert_eq!(key, "fib");
            assert_eq!(
                export_map.function,
                decoder.module.functions[export_map.index]
            );
        }
    }
}
