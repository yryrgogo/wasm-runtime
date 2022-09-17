use super::types::NumberType;
use crate::{
    instruction::Instruction,
    leb128::{decode_signed_leb128, decode_unsigned_leb128},
    module::{
        section::{
            CodeSectionNode, ExportSectionNode, FunctionSectionNode, SectionId, TypeSectionNode,
        },
        ModuleNode,
    },
    node::{
        BlockInstructionNode, BrIfInstructionNode, BrInstructionNode, CallInstructionNode,
        CodeNode, ElseInstructionNode, EndInstructionNode, ExportDescNode, ExportNode,
        ExportTypeNode, ExpressionNode, FunctionTypeNode, GetLocalInstructionNode,
        I32AddInstructionNode, I32ConstInstructionNode, I32EqzInstructionNode,
        I32GeSInstructionNode, I32LtSInstructionNode, I32LtUInstructionNode,
        I32RemSInstructionNode, I32RemUInstructionNode, I32SubInstructionNode, IfInstructionNode,
        InstructionNode, LocalEntryNode, LoopInstructionNode, ResultTypeNode,
        SetLocalInstructionNode,
    },
    types::{BlockType, ValueType},
};
use std::error::Error;

pub struct Parser {}

impl Parser {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {})
    }

    pub fn parse(&self, bytes: &mut Vec<u8>) -> Result<ModuleNode, Box<dyn Error>> {
        let (magic, version) = self.module_header(bytes).expect("Invalid header");
        let mut module = ModuleNode::new(magic.try_into().unwrap(), version.try_into().unwrap())
            .unwrap_or_else(|_| panic!("Invalid module"));

        if bytes.len() == 0 {
            return Ok(module);
        }

        while bytes.len() > 0 {
            self.section(bytes, &mut module)
                .expect("Failed to parse section");
        }

        Ok(module)
    }

    fn module_header(&self, bytes: &mut Vec<u8>) -> Result<(Vec<u8>, Vec<u8>), Box<dyn Error>> {
        let magic_bytes = bytes[0..4].to_vec();
        let version = bytes[4..8].to_vec();
        *bytes = bytes[8..].to_vec();
        Ok((magic_bytes, version))
    }

    fn section(&self, bytes: &mut Vec<u8>, module: &mut ModuleNode) -> Result<(), Box<dyn Error>> {
        let id = Parser::read_u8(bytes).expect("Failed to parse section id");
        let (size, _) = Parser::read_u32(bytes).expect("Failed to parse section size");
        let mut section_bytes = bytes[0..(size as usize)].to_vec();
        (*bytes).drain(0..(size as usize));

        dbg!("section id: {}, size: {}", id, size);

        match SectionId::from(id) {
            SectionId::CustomSectionId => todo!("Custom section"),
            SectionId::TypeSectionId => {
                let section = self
                    .type_section(&mut section_bytes)
                    .expect("Failed to parse type section");
                module.set_type_section(section);
            }
            SectionId::ImportSectionId => todo!("import section"),
            SectionId::FunctionSectionId => {
                let section = self
                    .function_section(&mut section_bytes)
                    .expect("Failed to parse function section");
                module.set_function_section(section);
            }
            SectionId::GlobalSectionId => todo!("global section"),
            SectionId::ExportSectionId => {
                let section = self
                    .export_section(&mut section_bytes)
                    .expect("Failed to parse export section");
                module.set_export_section(section);
            }
            SectionId::StartSectionId => todo!("start section"),
            SectionId::CodeSectionId => {
                let section = self
                    .code_section(&mut section_bytes)
                    .expect("Failed to parse code section");
                module.set_code_section(section);
            }
            SectionId::ElementSectionId => todo!("element section"),
            SectionId::DataSectionId => todo!("data section"),
        };
        Ok(())
    }

    /// type section = section1(vec((functype)*))
    fn type_section(&self, bytes: &mut Vec<u8>) -> Result<TypeSectionNode, Box<dyn Error>> {
        let mut function_types: Vec<FunctionTypeNode> = vec![];
        let (count, _) = Parser::read_u32(bytes).expect("Failed to parse vector size");

        for _ in 0..count {
            let function_type = self
                .function_type(bytes)
                .expect("Failed to parse function type");
            function_types.push(function_type);
        }

        Ok(TypeSectionNode { function_types })
    }

    /// function section = section3(vec((typeidx)*))
    fn function_section(&self, bytes: &mut Vec<u8>) -> Result<FunctionSectionNode, Box<dyn Error>> {
        let mut type_indexes: Vec<u32> = vec![];
        let (count, _) = Parser::read_u32(bytes).expect("Failed to parse vector size");

        for _ in 0..count {
            let (type_index, _) = Parser::read_u32(bytes).expect("Failed to parse type index");
            type_indexes.push(type_index);
        }

        Ok(FunctionSectionNode { type_indexes })
    }

    /// export section = section7(vec((export)*))
    fn export_section(&self, bytes: &mut Vec<u8>) -> Result<ExportSectionNode, Box<dyn Error>> {
        let (count, _) = Parser::read_u32(bytes).expect("Failed to parse vector size");

        let mut exports: Vec<ExportNode> = vec![];
        for _ in 0..count {
            let (name_size, _) = Parser::read_u32(bytes).expect("Failed to parse name size");
            let name_bytes =
                Parser::read_bytes(bytes, name_size as usize).expect("Failed to parse name bytes");
            let export_desc = self
                .export_desc(bytes)
                .expect("Failed to parse export desc");

            exports.push(ExportNode {
                name: String::from_utf8(name_bytes).unwrap(),
                export_desc,
            });
        }

        Ok(ExportSectionNode { exports })
    }

    fn export_desc(&self, bytes: &mut Vec<u8>) -> Result<ExportDescNode, Box<dyn Error>> {
        let id = Parser::read_u8(bytes).expect("Failed to parse export desc id");
        let (index, _) = Parser::read_u32(bytes).expect("Failed to parse export desc index");

        Ok(ExportDescNode {
            export_type: ExportTypeNode::from(id),
            index: index,
        })
    }

    /// code section = section10(vec((code)*))
    fn code_section(&self, bytes: &mut Vec<u8>) -> Result<CodeSectionNode, Box<dyn Error>> {
        let (count, _) = Parser::read_u32(bytes).expect("Failed to parse vector size");
        let mut bodies: Vec<CodeNode> = vec![];

        for _ in 0..count {
            let body = self.code(bytes).expect("Failed to parse function body");
            bodies.push(body);
        }

        Ok(CodeSectionNode { bodies })
    }

    fn code(&self, bytes: &mut Vec<u8>) -> Result<CodeNode, Box<dyn Error>> {
        let (function_body_size, _) =
            Parser::read_u32(bytes).expect("Failed to parse function body size");

        let (local_count, _) =
            Parser::read_u32(bytes).expect("Failed to parse function body local count");
        let mut local_entries: Vec<LocalEntryNode> = vec![];

        for _ in 0..local_count {
            let local_entry = self
                .local_entry(bytes)
                .expect("Failed to parse local entry");
            local_entries.push(local_entry);
        }

        let expr = self
            .expression(bytes, None, &mut 0)
            .expect("Failed to parse expression");

        Ok(CodeNode {
            function_body_size,
            local_count,
            locals: local_entries,
            expr,
        })
    }

    fn local_entry(&self, bytes: &mut Vec<u8>) -> Result<LocalEntryNode, Box<dyn Error>> {
        let (count, _) = Parser::read_u32(bytes).expect("Failed to parse local entry count");

        let number_type = self
            .number_type(bytes)
            .expect("Failed to parse number type");

        Ok(LocalEntryNode {
            count,
            val_type: ValueType::Number(number_type),
        })
    }

    fn expression(
        &self,
        bytes: &mut Vec<u8>,
        default_instructions: impl IntoIterator<Item = InstructionNode>,
        size: &mut u32,
    ) -> Result<ExpressionNode, Box<dyn Error>> {
        let mut instructions: Vec<InstructionNode> = vec![];
        default_instructions
            .into_iter()
            .for_each(|i| instructions.push(i));

        loop {
            let instruction = self
                .instruction(bytes)
                .expect("Failed to parse instruction");
            *size += 1;
            match instruction {
                InstructionNode::End(end_instr) => {
                    instructions.push(InstructionNode::End(end_instr));
                    break;
                }
                InstructionNode::Else(else_instr) => {
                    instructions.push(InstructionNode::Else(else_instr));
                    break;
                }
                _ => {
                    instructions.push(instruction);
                }
            }
        }

        Ok(ExpressionNode { instructions })
    }

    fn instruction(&self, bytes: &mut Vec<u8>) -> Result<InstructionNode, Box<dyn Error>> {
        let opcode = Parser::read_u8(bytes).expect("Failed to parse opcode");
        let instruction = Instruction::from(opcode);

        match instruction {
            Instruction::Unreachable => todo!(),
            Instruction::Nop => todo!(),
            Instruction::Block => {
                let block_type = self.block_type(bytes).expect("Failed to parse block type");
                let mut size = 0;
                let expr = self
                    .expression(bytes, None, &mut size)
                    .expect("Failed to parse expression");
                Ok(InstructionNode::Block(BlockInstructionNode::new(
                    block_type, expr, size,
                )))
            }
            Instruction::Loop => {
                let block_type = self.block_type(bytes).expect("Failed to parse block type");
                let mut size = 0;
                let expr = self
                    .expression(bytes, None, &mut size)
                    .expect("Failed to parse expression");
                Ok(InstructionNode::Loop(LoopInstructionNode::new(
                    block_type, expr, size,
                )))
            }
            Instruction::If => {
                let block_type = self.block_type(bytes).expect("Failed to parse block type");
                let mut size = 0;
                let mut then_expr = self
                    .expression(bytes, None, &mut size)
                    .expect("Failed to parse if-then expression");
                let last_instr = then_expr
                    .instructions
                    .pop()
                    .unwrap_or_else(|| panic!("Failed to pop if-then instruction"));

                match last_instr {
                    InstructionNode::Else(_) => {
                        let else_expr = self
                            .expression(bytes, [last_instr.clone()], &mut size)
                            .expect("Failed to parse if-else expression");
                        Ok(InstructionNode::If(IfInstructionNode::new(
                            block_type,
                            then_expr,
                            Some(else_expr),
                            size,
                        )))
                    }
                    InstructionNode::End(_) => {
                        then_expr.instructions.push(last_instr);
                        Ok(InstructionNode::If(IfInstructionNode::new(
                            block_type, then_expr, None, size,
                        )))
                    }
                    _ => panic!("Invalid if-then expression"),
                }
            }
            Instruction::Else => {
                let else_instr = ElseInstructionNode::default();
                Ok(InstructionNode::Else(else_instr))
            }
            Instruction::End => Ok(InstructionNode::End(EndInstructionNode::default())),
            Instruction::Br => {
                let (depth, _) = Parser::read_u32(bytes).expect("Failed to parse br depth");
                Ok(InstructionNode::Br(BrInstructionNode::new(depth as usize)))
            }
            Instruction::BrIf => {
                let (depth, _) = Parser::read_u32(bytes).expect("Failed to parse br if depth");
                Ok(InstructionNode::BrIf(BrIfInstructionNode::new(
                    depth as usize,
                )))
            }
            Instruction::BrTable => todo!(),
            Instruction::Return => todo!(),
            Instruction::Call => {
                let (index, _) = Parser::read_u32(bytes).expect("Failed to parse call index");
                Ok(InstructionNode::Call(CallInstructionNode::new(index)))
            }
            Instruction::CallIndirect => todo!(),
            Instruction::Drop => todo!(),
            Instruction::Select => todo!(),
            Instruction::GetLocal => {
                let (index, _) = Parser::read_u32(bytes).expect("Failed to parse get local index");
                Ok(InstructionNode::GetLocal(GetLocalInstructionNode::new(
                    index,
                )))
            }
            Instruction::SetLocal => {
                let (index, _) = Parser::read_u32(bytes).expect("Failed to parse set local index");
                Ok(InstructionNode::SetLocal(SetLocalInstructionNode::new(
                    index,
                )))
            }
            Instruction::TeeLocal => todo!(),
            Instruction::GetGlobal => todo!(),
            Instruction::SetGlobal => todo!(),
            Instruction::I32Load => todo!(),
            Instruction::I64Load => todo!(),
            Instruction::F32Load => todo!(),
            Instruction::F64Load => todo!(),
            Instruction::I32Load8S => todo!(),
            Instruction::I32Load8U => todo!(),
            Instruction::I32Load16S => todo!(),
            Instruction::I32Load16U => todo!(),
            Instruction::I64Load8S => todo!(),
            Instruction::I64Load8U => todo!(),
            Instruction::I64Load16S => todo!(),
            Instruction::I64Load16U => todo!(),
            Instruction::I64Load32S => todo!(),
            Instruction::I64Load32U => todo!(),
            Instruction::I32Store => todo!(),
            Instruction::I64Store => todo!(),
            Instruction::F32Store => todo!(),
            Instruction::F64Store => todo!(),
            Instruction::I32Store8 => todo!(),
            Instruction::I32Store16 => todo!(),
            Instruction::I64Store8 => todo!(),
            Instruction::I64Store16 => todo!(),
            Instruction::I64Store32 => todo!(),
            Instruction::CurrentMemory => todo!(),
            Instruction::GrowMemory => todo!(),
            Instruction::I32Const => {
                let (value, _) = Parser::read_i32(bytes).expect("Failed to parse const i32");
                let node = InstructionNode::I32Const(I32ConstInstructionNode::new(value));
                Ok(node)
            }
            Instruction::I64Const => todo!(),
            Instruction::F32Const => todo!(),
            Instruction::F64Const => todo!(),
            Instruction::I32Eqz => {
                let node = InstructionNode::I32Eqz(I32EqzInstructionNode::default());
                Ok(node)
            }
            Instruction::I32Eq => todo!(),
            Instruction::I32Ne => todo!(),
            Instruction::I32LtS => {
                let node = InstructionNode::I32LtS(I32LtSInstructionNode::default());
                Ok(node)
            }
            Instruction::I32LtU => {
                let node = InstructionNode::I32LtU(I32LtUInstructionNode::default());
                Ok(node)
            }
            Instruction::I32GtS => todo!(),
            Instruction::I32GtU => todo!(),
            Instruction::I32LeS => todo!(),
            Instruction::I32LeU => todo!(),
            Instruction::I32GeS => {
                let node = InstructionNode::I32GeS(I32GeSInstructionNode::default());
                Ok(node)
            }
            Instruction::I32GeU => todo!(),
            Instruction::I64Eqz => todo!(),
            Instruction::I64Eq => todo!(),
            Instruction::I64Ne => todo!(),
            Instruction::I64LtS => todo!(),
            Instruction::I64LtU => todo!(),
            Instruction::I64GtS => todo!(),
            Instruction::I64GtU => todo!(),
            Instruction::I64LeS => todo!(),
            Instruction::I64LeU => todo!(),
            Instruction::I64GeS => todo!(),
            Instruction::I64GeU => todo!(),
            Instruction::F32Eq => todo!(),
            Instruction::F32Ne => todo!(),
            Instruction::F32Lt => todo!(),
            Instruction::F32Gt => todo!(),
            Instruction::F32Le => todo!(),
            Instruction::F32Ge => todo!(),
            Instruction::F64Eq => todo!(),
            Instruction::F64Ne => todo!(),
            Instruction::F64Lt => todo!(),
            Instruction::F64Gt => todo!(),
            Instruction::F64Le => todo!(),
            Instruction::F64Ge => todo!(),
            Instruction::I32Clz => todo!(),
            Instruction::I32Ctz => todo!(),
            Instruction::I32Popcnt => todo!(),
            Instruction::I32Add => {
                let node = InstructionNode::I32Add(I32AddInstructionNode::default());
                Ok(node)
            }
            Instruction::I32Sub => {
                let node = InstructionNode::I32Sub(I32SubInstructionNode::default());
                Ok(node)
            }
            Instruction::I32Mul => todo!(),
            Instruction::I32DivS => todo!(),
            Instruction::I32DivU => todo!(),
            Instruction::I32RemS => {
                let node = InstructionNode::I32RemS(I32RemSInstructionNode::default());
                Ok(node)
            }
            Instruction::I32RemU => {
                let node = InstructionNode::I32RemU(I32RemUInstructionNode::default());
                Ok(node)
            }
            Instruction::I32And => todo!(),
            Instruction::I32Or => todo!(),
            Instruction::I32Xor => todo!(),
            Instruction::I32Shl => todo!(),
            Instruction::I32ShrS => todo!(),
            Instruction::I32ShrU => todo!(),
            Instruction::I32Rotl => todo!(),
            Instruction::I32Rotr => todo!(),
            Instruction::I64Clz => todo!(),
            Instruction::I64Ctz => todo!(),
            Instruction::I64Popcnt => todo!(),
            Instruction::I64Add => todo!(),
            Instruction::I64Sub => todo!(),
            Instruction::I64Mul => todo!(),
            Instruction::I64DivS => todo!(),
            Instruction::I64DivU => todo!(),
            Instruction::I64RemS => todo!(),
            Instruction::I64RemU => todo!(),
            Instruction::I64And => todo!(),
            Instruction::I64Or => todo!(),
            Instruction::I64Xor => todo!(),
            Instruction::I64Shl => todo!(),
            Instruction::I64ShrS => todo!(),
            Instruction::I64ShrU => todo!(),
            Instruction::I64Rotl => todo!(),
            Instruction::I64Rotr => todo!(),
            Instruction::F32Abs => todo!(),
            Instruction::F32Neg => todo!(),
            Instruction::F32Ceil => todo!(),
            Instruction::F32Floor => todo!(),
            Instruction::F32Trunc => todo!(),
            Instruction::F32Nearest => todo!(),
            Instruction::F32Sqrt => todo!(),
            Instruction::F32Add => todo!(),
            Instruction::F32Sub => todo!(),
            Instruction::F32Mul => todo!(),
            Instruction::F32Div => todo!(),
            Instruction::F32Min => todo!(),
            Instruction::F32Max => todo!(),
            Instruction::F32Copysign => todo!(),
            Instruction::F64Abs => todo!(),
            Instruction::F64Neg => todo!(),
            Instruction::F64Ceil => todo!(),
            Instruction::F64Floor => todo!(),
            Instruction::F64Trunc => todo!(),
            Instruction::F64Nearest => todo!(),
            Instruction::F64Sqrt => todo!(),
            Instruction::F64Add => todo!(),
            Instruction::F64Sub => todo!(),
            Instruction::F64Mul => todo!(),
            Instruction::F64Div => todo!(),
            Instruction::F64Min => todo!(),
            Instruction::F64Max => todo!(),
            Instruction::F64Copysign => todo!(),
        }
    }

    /// functype = 0x60 (result type) (result type)
    fn function_type(&self, bytes: &mut Vec<u8>) -> Result<FunctionTypeNode, Box<dyn Error>> {
        let header = Parser::read_u8(bytes).expect("Failed to parse function type header");
        let params = self
            .result_types(bytes)
            .expect("Failed to parse value type");
        let returns = self
            .result_types(bytes)
            .expect("Failed to parse value type");

        let function_type_node: FunctionTypeNode = FunctionTypeNode::new(params, returns);
        function_type_node.validate_header(header);

        Ok(function_type_node)
    }

    /// result type = vec((value type)*)
    fn result_types(&self, bytes: &mut Vec<u8>) -> Result<ResultTypeNode, Box<dyn Error>> {
        let (count, _) = Parser::read_u32(bytes).expect("Failed to read count");

        let mut node = ResultTypeNode { val_types: vec![] };
        for _ in 0..count {
            let number_type = self
                .number_type(bytes)
                .expect("Failed to parse number type");
            node.val_types.push(ValueType::Number(number_type));
        }
        Ok(node)
    }

    fn number_type(&self, bytes: &mut Vec<u8>) -> Result<NumberType, Box<dyn Error>> {
        let byte = Parser::read_u8(bytes).expect("Failed to read number type id");
        Ok(NumberType::from(byte))
    }

    fn block_type(&self, bytes: &mut Vec<u8>) -> Result<BlockType, Box<dyn Error>> {
        let byte = Parser::read_u8(bytes).expect("Failed to read block type id");
        let block_type = BlockType::from(byte);
        Ok(block_type)
    }

    pub fn read_u8(bytes: &mut Vec<u8>) -> Result<u8, Box<dyn Error>> {
        let byte = bytes[0];
        (*bytes).drain(0..1);
        Ok(byte)
    }

    pub fn read_u32(bytes: &mut Vec<u8>) -> Result<(u32, u32), Box<dyn Error>> {
        decode_unsigned_leb128(bytes)
    }

    pub fn read_i32(bytes: &mut Vec<u8>) -> Result<(i32, u32), Box<dyn Error>> {
        decode_signed_leb128(bytes)
    }

    pub fn read_bytes(bytes: &mut Vec<u8>, size: usize) -> Result<Vec<u8>, Box<dyn Error>> {
        let b = bytes[0..size].to_vec();
        (*bytes).drain(0..size);
        Ok(b)
    }
}

#[cfg(test)]
mod leb128_tests {
    use super::*;

    #[test]
    fn read_u32_case1() {
        let mut bytes = vec![229, 142, 38, 0, 0, 0, 0, 0];
        let (value, size) = Parser::read_u32(&mut bytes).expect("Invalid u32");
        assert_eq!(value, 624485);
        assert_eq!(size, 3);
    }

    #[test]
    fn read_u32_case2() {
        let mut bytes = vec![0x80, 0x80, 0xC0, 0x00, 0x0B];
        let (value, size) = Parser::read_u32(&mut bytes).expect("Invalid u32");

        assert_eq!(value, 1048576);
        assert_eq!(size, 4);
    }

    #[test]
    fn test_read_i32() {
        let mut bytes = vec![127, 0, 0, 0, 0, 0, 0, 0];
        let (value, size) = Parser::read_i32(&mut bytes).expect("Invalid i32");

        assert_eq!(value, -1);
        assert_eq!(size, 1);
    }
}
