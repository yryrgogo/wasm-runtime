use crate::reader::{WasmBinaryReader, LEB128_MAX_BITS};
use crate::{
    export::ExportMap,
    import::ImportMap,
    module::{
        function::{Block, Function},
        function_type::FunctionType,
        number::NumberType,
        opcode::OpCode,
        section::{ExternalKind, SectionId, TypeSection},
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
        path: Option<&String>,
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

        println!("{:#?}", self.module);
    }

    pub fn decode_header(&mut self) {
        let header = String::from_utf8(Vec::from(self.reader.read_header()))
            .unwrap_or_else(|_| panic!("ヘッダの u8 -> String 変換に失敗しました。"));
        if !self.module.valid_header(&header) {
            panic!("Invalid wasm header: {}", header);
        }
    }

    pub fn decode_section(&mut self) {
        println!("\n# Section Decode Start!\n");
        loop {
            match self.decode_section_id() {
                Some(section_id) => self.decode_section_body(section_id).unwrap_or_else(|err| {
                    panic!("Section Body のでコードに失敗しました。 {:?}", err)
                }),
                None => break,
            }
        }

        // FIXME: Export Section 時点では関数の内容がデコードされていないため、Code Section 完了後に無理やり置き換えている。
        // exports には可変参照を格納して、Code Section で順次書き換えていけるとよいのだが、Lifetime を解決できず断念
        for (_, export_map) in &mut self.module.exports {
            export_map.function = self.module.functions[export_map.index].clone();
        }

        println!("\n# Section Decode Complete!\n");
    }

    pub fn decode_section_id(&mut self) -> Option<u8> {
        self.reader.read_next_byte()
    }

    pub fn decode_section_body(&mut self, section_id: u8) -> Result<(), Box<dyn Error>> {
        let [section_size, decoded_size] = self.reader.read_unsigned_leb128();

        println!("------------------------");
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
            SectionId::ImportSectionId => self.decode_import_section(),
            SectionId::FunctionSectionId => self.decode_function_section(),
            SectionId::ExportSectionId => self.decode_export_section(),
            SectionId::StartSectionId => self.decode_start_section(),
            SectionId::CodeSectionId => self.decode_code_section(),
            SectionId::StartSectionId => todo!(),
        }

        println!("");

        Ok(())
    }

    /// Decode TypeSection
    ///
    /// Reference
    /// - https://github.com/WebAssembly/design/blob/main/BinaryEncoding.md#type-section
    /// - https://github.com/WebAssembly/design/blob/main/BinaryEncoding.md#func_type
    /// ```
    /// [
    ///  type entry count,
    ///  type section header,
    ///  function parameter count,
    ///  function parameter type,
    ///  ...,
    ///  return value type
    /// ]
    /// ```
    fn decode_type_section(&mut self) {
        println!(
            "
#==============#
# Type Section #
#==============#
        "
        );

        let [type_entry_count, size] = self.reader.read_unsigned_leb128();
        println!(
            "type entry count: {} Decoded size: {}",
            type_entry_count, size
        );

        for s_i in 0..type_entry_count {
            println!("* type entry {}", s_i + 1);

            TypeSection::validate_type_entry_header(
                self.reader.read_next_byte().unwrap_or_else(|| {
                    panic!("TypeSection の type entry header が見つかりません。")
                }),
            );

            let mut func_type = FunctionType::default();

            let [parameter_count, _] = self.reader.read_unsigned_leb128();
            for p_i in 0..parameter_count {
                let num_type = self.decode_type().unwrap();
                println!("Parameter {} / Type {:?}", p_i + 1, num_type);
                func_type.parameters.push(num_type);
            }

            let [result_count, _] = self.reader.read_unsigned_leb128();

            // NOTE: 202203時点の仕様では戻り値は1つまで
            assert!(result_count <= 1);

            for r_i in 0..result_count {
                let value = self.decode_type().unwrap();
                println!("Result {} / Type {:?}", r_i + 1, value);
                func_type.results.push(value);
            }
            self.module.function_types.push(func_type);
        }
    }

    /// Decode ImportSection
    ///
    /// Reference
    /// - https://github.com/WebAssembly/design/blob/main/BinaryEncoding.md#import-section
    fn decode_import_section(&mut self) {
        println!(
            "
#================#
# Import Section #
#================#
        "
        );

        let [import_entry_count, size] = self.reader.read_unsigned_leb128();
        println!(
            "import entry count: {} Decoded size: {}",
            import_entry_count, size
        );

        for _ in 0..import_entry_count {
            let [module_name_size, size] = self.reader.read_unsigned_leb128();
            let buf = self.reader.read_bytes(module_name_size);
            let module_name = std::str::from_utf8(&buf).unwrap();
            println!("import module name: {}, size: {}", module_name, size);

            let [import_name_size, size] = self.reader.read_unsigned_leb128();
            let buf = self.reader.read_bytes(import_name_size);
            let import_name = std::str::from_utf8(&buf).unwrap();
            println!("import field name: {}, size: {}", import_name, size);

            let kind = self
                .reader
                .read_next_byte()
                .unwrap_or_else(|| panic!("Import Section の kind byte が見つかりません"));

            let module_import_name = format!("{}.{}", module_name, import_name);

            match ExternalKind::from_usize(kind).unwrap() {
                ExternalKind::Func => {
                    if self.module.exports.contains_key(&module_import_name) {
                        panic!("{} key already exists", &module_import_name);
                    }
                    let [func_index, _] = self.reader.read_unsigned_leb128();
                    println!("import function index: {}", func_index);

                    self.module.imports.insert(
                        module_import_name,
                        ImportMap {
                            index: func_index,
                            function_type: self.module.function_types[func_index].clone(),
                        },
                    );
                }
                ExternalKind::Table => todo!(),
                ExternalKind::LinearMemory => todo!(),
                ExternalKind::GlobalVariable => todo!(),
            }
        }
    }

    /// Function Section
    ///
    /// Reference
    /// - https://github.com/WebAssembly/design/blob/main/BinaryEncoding.md#function-section
    ///
    /// ```
    /// [
    ///  function count,
    ///  function type index,
    /// ]
    /// ```
    fn decode_function_section(&mut self) {
        println!(
            "
#==================#
# Function Section #
#==================#
        "
        );

        if self.module.imports.len() > 0 {
            for i in 0..self.module.imports.len() {
                let func_type = self.module.function_types[i].clone();
                self.module
                    .functions
                    .push(Function::new(func_type, Some(self.module.functions.len())))
            }
        }

        let [local_function_count, _] = self.reader.read_unsigned_leb128();
        println!("local function count: {}", local_function_count);

        for _ in 0..local_function_count {
            let [func_type_index, _] = self.reader.read_unsigned_leb128();
            println!("function type index: {}", func_type_index);
            let func_type = self.module.function_types[func_type_index].clone();
            self.module
                .functions
                .push(Function::new(func_type, Some(self.module.functions.len())))
        }
    }

    /// Export Section
    ///
    /// Reference
    /// - https://github.com/WebAssembly/design/blob/main/BinaryEncoding.md#export-section
    /// - https://github.com/WebAssembly/design/blob/main/BinaryEncoding.md#function-section
    ///
    /// ```
    /// [
    ///  export count,
    ///  export name size,
    ///  export kind （Export される値のタイプ）,
    ///  TODO: この index は Function Section で定義されている index と異なる。ここで返される index は import function の数 +1（0始まりなので、import function が3つあれば3になる）になっているっぽい？怪しいのでいろいろな wat を変換しながら検証に必要あり
    ///  export kind index （Export される値のインデックス。Function の場合は Function の index）
    /// ]
    /// ```
    fn decode_export_section(&mut self) {
        println!(
            "
#================#
# Export Section #
#================#
        "
        );

        let [export_entry_count, _] = self.reader.read_unsigned_leb128();

        println!("Export entry count: {}", export_entry_count);
        for _ in 0..export_entry_count {
            let [name_size, _] = self.reader.read_unsigned_leb128();
            let name_buf = self.reader.read_bytes(name_size);
            let name = std::str::from_utf8(&name_buf).unwrap();
            let kind = self
                .reader
                .read_next_byte()
                .unwrap_or_else(|| panic!("Export Section の kind byte が見つかりません"));

            match ExternalKind::from_usize(kind).unwrap() {
                ExternalKind::Func => {
                    if self.module.exports.contains_key(name) {
                        panic!("{} key already exists", name);
                    }

                    // TODO: 要検証部分 export index の定義がよくわかってない
                    let [export_func_idx, _] = self.reader.read_unsigned_leb128();
                    println!("export function index: {}", export_func_idx);

                    self.module.exports.insert(
                        name.to_string(),
                        ExportMap {
                            index: export_func_idx,
                            function: self
                                .module
                                .functions
                                .get(export_func_idx)
                                .unwrap_or_else(|| {
                                    panic!("function index: {} name: {}", export_func_idx, name)
                                })
                                .clone(),
                        },
                    );
                }
                ExternalKind::Table => todo!(),
                ExternalKind::LinearMemory => todo!(),
                ExternalKind::GlobalVariable => todo!(),
            }
        }
    }

    /// Start Section
    ///
    /// Reference
    /// - https://github.com/WebAssembly/design/blob/main/BinaryEncoding.md#start-section
    /// ```
    fn decode_start_section(&mut self) {
        println!(
            "
#===============#
# Start Section #
#===============#
        "
        );

        let [index, _] = self.reader.read_unsigned_leb128();

        println!("Start Index: {}", index);
    }

    /// Code Section
    ///
    /// Reference
    /// - https://github.com/WebAssembly/design/blob/main/BinaryEncoding.md#code-section
    ///
    /// ```
    /// [
    ///   local variable and bytecodes pair count,
    ///   local variable and bytecodes pair size,
    ///   local variable count,
    ///   local variable type count,
    ///   local variable type,
    ///   ...
    ///   byte code of the function...
    /// ]
    /// ```
    fn decode_code_section(&mut self) {
        println!(
            "
#==============#
# Code Section #
#==============#
        "
        );

        let [function_body_count, _] = self.reader.read_unsigned_leb128();
        println!("function body count: {}", function_body_count);

        for function_body_idx in 0..function_body_count {
            self.decode_code_section_function_body(function_body_idx);
            self.analyze_code_section_function_body_code(function_body_idx);
        }
    }

    /// Reference
    /// - https://github.com/WebAssembly/design/blob/main/BinaryEncoding.md#function-bodies
    fn decode_code_section_function_body(&mut self, code_idx: usize) {
        println!("Code index {}", code_idx);

        let [body_size, _] = self.reader.read_unsigned_leb128();
        println!("Function body size: {}", body_size);

        let mut local_var_byte_size: usize = 0;
        let [local_var_count, size] = self.reader.read_unsigned_leb128();
        local_var_byte_size += size;
        println!("local var count: {}", local_var_count);

        for _ in 0..local_var_count {
            let local_var_type_count_byte_size = self.decode_code_section_body_local_var(code_idx);
            local_var_byte_size += local_var_type_count_byte_size;
            local_var_byte_size += 1;
        }
        let bytecodes_buf = self.reader.read_bytes(body_size - local_var_byte_size);
        self.module.functions[code_idx].bytecodes = bytecodes_buf.to_vec();
    }

    ///
    /// Reference
    /// - https://github.com/WebAssembly/design/blob/main/BinaryEncoding.md#local-entry
    fn decode_code_section_body_local_var(&mut self, func_idx: usize) -> usize {
        let [local_var_type_count, local_var_type_count_byte_size] =
            self.reader.read_unsigned_leb128();
        let local_var_type = self.decode_type().unwrap();
        println!(
            "local var type: {} count: {:x}",
            local_var_type.inspect(),
            local_var_type_count
        );

        self.module.functions[func_idx].local_vars = vec![local_var_type; local_var_type_count];
        local_var_type_count_byte_size
    }

    fn analyze_code_section_function_body_code(&mut self, func_idx: usize) {
        let func = self
            .module
            .functions
            .get(func_idx)
            .unwrap_or_else(|| panic!("index {} の関数が見つかりません。", func_idx));
        let mut bytecodes = self.module.functions[func_idx].bytecodes.clone();
        let mut blocks: HashMap<usize, Block> = HashMap::new();
        let mut block_stack = vec![Block::new(2, func.func_type.results.clone(), 0, None)];

        bytecodes.reverse();

        loop {
            if bytecodes.len() == 0 {
                break;
            }
            match self.find_next_structured_instruction(&mut bytecodes) {
                Some(structured_instruction) => {
                    let idx = self.module.functions[func_idx].bytecodes.len() - bytecodes.len() - 1;

                    match OpCode::from_byte(structured_instruction) {
                        OpCode::End => {
                            let mut block = block_stack.pop().unwrap();
                            block.end_idx = idx;

                            println!(
                                "  [Structured Instruction] block {}-{}",
                                block.start_idx, block.end_idx
                            );

                            blocks.insert(block.start_idx, block);
                        }
                        OpCode::Unreachable => {
                            break;
                        }
                        op => {
                            println!("  [Structured Instruction] OpCode: {:?}", op);
                            let opcode = bytecodes.pop().unwrap_or_else(|| {
                                panic!("Block Section の arity 読み込みに失敗しました。")
                            });
                            let arity: Vec<NumberType> = if opcode == 0x40 {
                                vec![]
                            } else {
                                let v = NumberType::decode_byte(opcode).unwrap_or_else(|| {
                                    panic!("NumberType に渡した byte 値が不正です。")
                                });
                                vec![v]
                            };
                            println!("  [Structured Instruction] {:?} arity: {:?}", op, arity);
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

    fn decode_type(&mut self) -> Option<NumberType> {
        let byte = self
            .reader
            .read_next_byte()
            .unwrap_or_else(|| panic!("Value Type の byte 読み込みに失敗しました。"));
        NumberType::decode_byte(byte)
    }

    /// block, loop, if の index を解析するにあたり、各オペコードに続く byte をスキップする
    /// NOTE: ここでスキップ処理の漏れがあると、正しく解析できない
    fn find_next_structured_instruction(&mut self, bytecodes: &mut Vec<u8>) -> Option<u8> {
        let mut byte;
        loop {
            if bytecodes.len() == 0 {
                return None;
            }
            byte = bytecodes.pop().unwrap();
            match OpCode::from_byte(byte) {
                OpCode::Block | OpCode::Loop | OpCode::If | OpCode::End => {
                    // println!("block or loop or if or end");
                    break;
                }
                OpCode::Br | OpCode::BrIf => {
                    Decoder::decode_unsigned_leb128(bytecodes);
                }
                OpCode::Call => {
                    Decoder::decode_unsigned_leb128(bytecodes);
                }
                OpCode::GetLocal
                | OpCode::SetLocal
                | OpCode::TeeLocal
                | OpCode::GetGlobal
                | OpCode::SetGlobal => {
                    println!("OpCode: {:x} get/set local/global", byte);
                    Decoder::decode_unsigned_leb128(bytecodes);
                }
                OpCode::I32Const | OpCode::I64Const | OpCode::F32Const | OpCode::F64Const => {
                    println!("OpCode: {:x} Const", byte);
                    // TODO: f32を定義すると4byte, f64を定義すると8byteが後ろに続くっぽい？ https://github.com/WebAssembly/design/blob/main/Semantics.md#floating-point-operators
                    Decoder::decode_signed_leb128(bytecodes);
                }
                _ => {}
            };
        }

        Some(byte)
    }

    fn discard_section(&mut self, size: usize) -> Result<(), Box<dyn Error>> {
        self.reader.read_bytes(size);
        // let bytes = self.reader.read_bytes(size);
        // for b in bytes.clone() {
        //     println!("section byte: {}", b);
        // }
        Ok(())
    }

    fn decode_unsigned_leb128(buf: &mut Vec<u8>) -> [usize; 2] {
        let mut value: usize = 0;
        let mut shift: usize = 0;
        let mut byte_count: usize = 0;

        loop {
            let byte = buf.pop().unwrap();
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

    fn decode_signed_leb128(buf: &mut Vec<u8>) -> [usize; 2] {
        let mut value: usize = 0;
        let mut shift: usize = 0;
        let mut byte_count: usize = 0;

        loop {
            let byte = buf.pop().unwrap();
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
}

#[cfg(test)]
mod leb_tests {
    use super::*;

    #[test]
    fn can_read_unsigned_leb128_1() {
        let wasm_module = vec![229, 142, 38, 0, 0, 0, 0, 0];
        let mut decoder = Decoder::new(None, Some(wasm_module)).unwrap();
        let [value, size] = decoder.reader.read_unsigned_leb128();

        assert_eq!(value, 624485);
        assert_eq!(size, 3);
    }

    #[test]
    fn can_read_unsigned_leb128_2() {
        let wasm_module = vec![0x80, 0x80, 0xC0, 0x00, 0x0B];
        let mut decoder = Decoder::new(None, Some(wasm_module)).unwrap();
        let [value, size] = decoder.reader.read_unsigned_leb128();

        assert_eq!(value, 1048576);
        assert_eq!(size, 4);
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
mod decode_tests {
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
            vec![NumberType::decode_byte(0x7f).unwrap()]
        );
        assert_eq!(
            func_type.results,
            vec![NumberType::decode_byte(0x7f).unwrap()]
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

        decoder.decode_type_section();
        decoder.decode_function_section();
        decoder.decode_export_section();

        for (key, export_map) in decoder.module.exports {
            assert_eq!(key, "fib");
            assert_eq!(
                export_map.function,
                decoder.module.functions[export_map.index]
            );
        }
    }

    #[test]
    fn can_decode_code_section() {
        let wasm_module_type_section = vec![
            // Type Section
            0x01, 0x60, 0x01, 0x7f, 0x01, 0x7f, // Function Section
            0x01, 0x00, // Export Section
            0x01, 0x03, 0x66, 0x69, 0x62, 0x00, 0x00, // Code Section
            0x01, 0x32, 0x01, 0x03, 0x7f, 0x20, 0x00, 0x41, 0x02, 0x4f, 0x04, 0x40, 0x20, 0x00,
            0x41, 0x7f, 0x6a, 0x21, 0x01, 0x41, 0x01, 0x21, 0x00, 0x03, 0x40, 0x20, 0x00, 0x22,
            0x03, 0x20, 0x02, 0x6a, 0x21, 0x00, 0x20, 0x03, 0x21, 0x02, 0x20, 0x01, 0x41, 0x7f,
            0x6a, 0x22, 0x01, 0x0d, 0x00, 0x0b, 0x0b, 0x20, 0x00, 0x0b,
        ];
        let mut decoder = Decoder::new(None, Some(wasm_module_type_section)).unwrap();

        decoder.decode_type_section();
        decoder.decode_function_section();
        decoder.decode_export_section();
        decoder.decode_code_section();

        assert_eq!(
            decoder.module.functions[0].local_vars,
            vec![NumberType::Int32; 3]
        );

        for (idx, block) in decoder.module.functions[0].clone().blocks {
            match idx {
                0 => assert_eq!(block, Block::new(2, vec![NumberType::Int32], 0, Some(46))),
                5 => assert_eq!(block, Block::new(4, vec![], 5, Some(43))),
                18 => assert_eq!(block, Block::new(3, vec![], 18, Some(42))),
                _ => unreachable!("{}", idx),
            }
        }
    }
}
