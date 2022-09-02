use crate::types::ValueType;

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

#[derive(Debug)]
pub struct ExportDescNode {
    pub export_type: ExportType,
    pub index: u32,
}

#[derive(Debug)]
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
    GetLocal(GetLocalInstructionNode),
    SetLocal(SetLocalInstructionNode),
    // Unreachable,
    // Nop,
    // Block(BlockTypeNode),
    // Loop(BlockTypeNode),
    // If(BlockTypeNode),
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
