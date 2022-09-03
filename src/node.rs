use crate::types::{NumberType, ValueType};

// https://webassembly.github.io/spec/core/binary/types.html#function-types
#[derive(Debug)]
pub struct FunctionTypeNode {
    pub params: ResultTypeNode,
    pub returns: ResultTypeNode,
}
impl FunctionTypeNode {
    pub fn validate_header(header: u8) {
        const HEADER: u8 = 0x60;
        if header != HEADER {
            panic!("Invalid TypeSection header {}", header);
        }
    }
}

// https://webassembly.github.io/spec/core/binary/types.html#result-types
#[derive(Debug)]
pub struct ResultTypeNode {
    // TODO: replace to Value Types
    pub val_types: Vec<ValueType>,
}

#[derive(Debug)]
pub struct CodeNode {
    pub function_body_size: u32,
    pub local_count: u32,
    pub locals: Vec<LocalEntryNode>,
    pub expr: ExpressionNode,
}

#[derive(Debug)]
pub struct LocalEntryNode {
    pub count: u32,
    pub val_type: ValueType,
}

#[derive(Debug)]
pub struct ExpressionNode {
    pub instructions: Vec<InstructionNode>,
}

#[derive(Debug)]
pub struct ExportNode {
    pub name: Vec<u8>,
    pub export_desc: ExportDescNode,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ExportDescNode {
    pub export_type: ExportType,
    pub index: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ExportType {
    Function = 0x00,
    // Table = 0x01,
    // Memory = 0x02,
    // Global = 0x03,
}

impl From<u8> for ExportType {
    fn from(x: u8) -> ExportType {
        match x {
            0x00 => ExportType::Function,
            // 0x01 => ExportType::Table,
            // 0x02 => ExportType::Memory,
            // 0x03 => ExportType::Global,
            _ => unreachable!("{} is an invalid value in ExportType", x),
        }
    }
}

//
// instructions
//

#[derive(Debug)]
pub enum InstructionNode {
    I32Const(I32ConstInstructionNode),
    End(EndInstructionNode),
    Else(ElseInstructionNode),
    GetLocal(GetLocalInstructionNode),
    SetLocal(SetLocalInstructionNode),
    I32Add(I32AddInstructionNode),
    // I32Sub(I32SubInstructionNode),
    // I32RemU(I32RemUInstructionNode),
    // I32Shl(I32ShlInstructionNode),
    // I32Eqz(I32EqzInstructionNode),
    // I32Eq(I32EqInstructionNode),
    // I32LtS(I32LtSInstructionNode),
    // I32LtU(I32LtUInstructionNode),
    I32GeS(I32GeSInstructionNode),
    I32GeU(I32GeUInstructionNode),
    I32GtS(I32GtSInstructionNode),
    I32GtU(I32GtUInstructionNode),
    // I64Add(I32AddInstructionNode),
    // I64Sub(I32SubInstructionNode),
    // Unreachable,
    // Nop,
    // Block(BlockTypeNode),
    // Loop(BlockTypeNode),
    If(IfInstructionNode),
    // Else,
    // End,
    // Br(u32),
    // BrIf(u32),
    // BrTable(Vec<u32>, u32),
    // Return,
    // Call(u32),
}

#[derive(Debug)]
pub struct I32ConstInstructionNode {
    pub opcode: u8,
    pub value: i32,
}

#[derive(Debug)]
pub struct ElseInstructionNode {
    pub opcode: u8,
}

#[derive(Debug)]
pub struct EndInstructionNode {
    pub opcode: u8,
}

#[derive(Debug)]
pub struct GetLocalInstructionNode {
    pub opcode: u8,
    pub index: u32,
}

#[derive(Debug)]
pub struct SetLocalInstructionNode {
    pub opcode: u8,
    pub index: u32,
}

#[derive(Debug)]
pub struct I32AddInstructionNode {
    pub opcode: u8,
}
#[derive(Debug)]
pub struct I32SubInstructionNode {
    pub opcode: u8,
}

#[derive(Debug)]
pub struct I64AddInstructionNode {
    pub opcode: u8,
}

#[derive(Debug)]
pub struct I64SubInstructionNode {
    pub opcode: u8,
}

#[derive(Debug)]
pub struct I32RemUInstructionNode {
    pub opcode: u8,
}

#[derive(Debug)]
pub struct I32ShlInstructionNode {
    pub opcode: u8,
}

#[derive(Debug)]
pub struct I32EqzInstructionNode {
    pub opcode: u8,
}

#[derive(Debug)]
pub struct I32EqInstructionNode {
    pub opcode: u8,
}

#[derive(Debug)]
pub struct I32LtSInstructionNode {
    pub opcode: u8,
}

#[derive(Debug)]
pub struct I32LtUInstructionNode {
    pub opcode: u8,
}

#[derive(Debug)]
pub struct I32GeSInstructionNode {
    pub opcode: u8,
}

#[derive(Debug)]
pub struct I32GeUInstructionNode {
    pub opcode: u8,
}

#[derive(Debug)]
pub struct I32GtSInstructionNode {
    pub opcode: u8,
}

#[derive(Debug)]
pub struct I32GtUInstructionNode {
    pub opcode: u8,
}

#[derive(Debug)]
pub struct IfInstructionNode {
    pub opcode: u8,
    pub block_type: BlockType,
    pub then_expr: ExpressionNode,
    pub else_expr: Option<ExpressionNode>,
}

#[derive(Debug)]
pub enum BlockType {
    // Empty,
    ValType(ValueType),
    // S33,
}

impl From<u8> for BlockType {
    fn from(x: u8) -> BlockType {
        match x {
            // 0x40 => BlockType::Empty,
            0x7F => BlockType::ValType(ValueType::NumberType(NumberType::I32)),
            // 0x7E => BlockType::ValType(ValueType::NumberType(NumberType::I64)),
            // 0x7D => BlockType::ValType(ValueType::NumberType(NumberType::F32)),
            // 0x7C => BlockType::ValType(ValueType::NumberType(NumberType::F64)),
            // 0x70 => BlockType::S33,
            _ => unreachable!("{} is an invalid value in BlockType", x),
        }
    }
}
