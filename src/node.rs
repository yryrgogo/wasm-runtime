use crate::{
    leb128::{encode_i32_to_leb128, encode_u32_to_leb128},
    types::{BlockTypeNode, ValueTypeNode},
};

pub trait Node {
    fn size(&self) -> u32;
    fn encode(&self) -> Vec<u8>;
}

// https://webassembly.github.io/spec/core/binary/types.html#function-types
#[derive(Debug, Clone)]
pub struct FunctionTypeNode {
    pub header: u8,
    pub params: ResultTypeNode,
    pub returns: ResultTypeNode,
}

impl FunctionTypeNode {
    pub fn new(params: ResultTypeNode, returns: ResultTypeNode) -> Self {
        Self {
            header: 0x60,
            params,
            returns,
        }
    }

    pub fn validate_header(&self, header: u8) {
        if header != self.header {
            panic!("Invalid TypeSection header {}", header);
        }
    }
}

impl Node for FunctionTypeNode {
    fn size(&self) -> u32 {
        let mut size = 0;
        size += 1; // header
        size += 1; // count of params
        size += self.params.size();
        size += 1; // count of params
        size += self.returns.size();
        size
    }

    fn encode(&self) -> Vec<u8> {
        let mut buffer = vec![];
        buffer.push(self.header);
        buffer.extend(encode_u32_to_leb128(self.params.size()));
        buffer.extend(self.params.encode());
        buffer.extend(encode_u32_to_leb128(self.returns.size()));
        buffer.extend(self.returns.encode());
        buffer
    }
}

// https://webassembly.github.io/spec/core/binary/types.html#result-types
#[derive(Debug, Clone)]
pub struct ResultTypeNode {
    // TODO: replace to Value Types
    pub val_types: Vec<ValueTypeNode>,
}

impl Node for ResultTypeNode {
    fn size(&self) -> u32 {
        let mut size = 0;
        size += self.val_types.len();
        size as u32
    }

    fn encode(&self) -> Vec<u8> {
        let mut buffer = vec![];
        for val_type in self.val_types.iter() {
            match val_type {
                ValueTypeNode::NumberType(num) => {
                    buffer.extend(num.encode());
                }
            }
        }
        buffer
    }
}

#[derive(Debug, Clone)]
pub struct CodeNode {
    pub function_body_size: u32,
    pub local_count: u32,
    pub locals: Vec<LocalEntryNode>,
    pub expr: ExpressionNode,
}

impl Node for CodeNode {
    fn size(&self) -> u32 {
        let mut size = 0;
        size += encode_u32_to_leb128(self.function_body_size).len() as u32; // function body size
        size += encode_u32_to_leb128(self.local_count).len() as u32; // local entry count
        for local in self.locals.iter() {
            size += local.size();
        }
        size += self.expr.size();
        size
    }

    fn encode(&self) -> Vec<u8> {
        let mut buffer = vec![];
        buffer.extend(encode_u32_to_leb128(self.function_body_size));
        buffer.extend(encode_u32_to_leb128(self.local_count));
        for local in self.locals.iter() {
            buffer.extend(local.encode());
        }
        buffer.extend(self.expr.encode());
        buffer
    }
}

#[derive(Debug, Clone)]
pub struct LocalEntryNode {
    pub count: u32,
    pub val_type: ValueTypeNode,
}

impl Node for LocalEntryNode {
    fn size(&self) -> u32 {
        let mut size = 0;
        size += encode_u32_to_leb128(self.count).len() as u32; // count
        size += self.val_type.size(); // val_type
        size
    }

    fn encode(&self) -> Vec<u8> {
        let mut buffer = vec![];
        buffer.extend(encode_u32_to_leb128(self.count));
        buffer.extend(self.val_type.encode());
        buffer
    }
}

#[derive(Debug, Clone)]
pub struct ExpressionNode {
    pub instructions: Vec<InstructionNode>,
}

impl Node for ExpressionNode {
    fn size(&self) -> u32 {
        let mut size = 0;
        for instruction in self.instructions.iter() {
            size += instruction.size();
        }
        size
    }

    fn encode(&self) -> Vec<u8> {
        let mut buffer = vec![];
        for instruction in self.instructions.iter() {
            buffer.extend(instruction.encode());
        }
        buffer
    }
}

impl ExpressionNode {
    pub fn update_instruction(&mut self, index: usize, instruction: InstructionNode) {
        self.instructions[index] = instruction;
    }
}

#[derive(Debug, Clone)]
pub struct ExportNode {
    pub name: String,
    pub export_desc: ExportDescNode,
}

impl Node for ExportNode {
    fn size(&self) -> u32 {
        let mut size = 0;
        size += 1; // name size
        size += self.name.as_bytes().len() as u32;
        size += self.export_desc.size();
        size
    }

    fn encode(&self) -> Vec<u8> {
        let mut buffer = vec![];
        buffer.extend(encode_u32_to_leb128(self.name.len() as u32));
        buffer.extend(self.name.as_bytes());
        buffer.extend(self.export_desc.encode());
        buffer
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ExportDescNode {
    pub export_type: ExportTypeNode,
    pub index: u32,
}

impl Node for ExportDescNode {
    fn size(&self) -> u32 {
        let mut size = 0;
        size += 1; // export type
        size += encode_u32_to_leb128(self.index).len() as u32;
        size
    }

    fn encode(&self) -> Vec<u8> {
        let mut buffer = vec![];
        buffer.extend(self.export_type.encode());
        buffer.extend(encode_u32_to_leb128(self.index));
        buffer
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ExportTypeNode {
    Function = 0x00,
    // Table = 0x01,
    // Memory = 0x02,
    // Global = 0x03,
}

impl From<u8> for ExportTypeNode {
    fn from(x: u8) -> Self {
        match x {
            0x00 => ExportTypeNode::Function,
            // 0x01 => ExportType::Table,
            // 0x02 => ExportType::Memory,
            // 0x03 => ExportType::Global,
            _ => unreachable!("{} is an invalid value in ExportType", x),
        }
    }
}

impl Into<u8> for ExportTypeNode {
    fn into(self) -> u8 {
        match self {
            ExportTypeNode::Function => 0x00,
            // ExportType::Table => 0x01,
            // ExportType::Memory => 0x02,
            // ExportType::Global => 0x03,
        }
    }
}

impl Node for ExportTypeNode {
    fn size(&self) -> u32 {
        1
    }

    fn encode(&self) -> Vec<u8> {
        vec![(*self).into()]
    }
}

//
// instructions
//

#[derive(Debug, Clone)]
pub enum InstructionNode {
    Block(BlockInstructionNode),
    Loop(LoopInstructionNode),
    If(IfInstructionNode),
    Else(ElseInstructionNode),
    Br(BrInstructionNode),
    BrIf(BrIfInstructionNode),
    Call(CallInstructionNode),
    End(EndInstructionNode),
    I32Const(I32ConstInstructionNode),
    GetLocal(GetLocalInstructionNode),
    SetLocal(SetLocalInstructionNode),
    I32Add(I32AddInstructionNode),
    I32Sub(I32SubInstructionNode),
    // I32RemU(I32RemUInstructionNode),
    // I32Shl(I32ShlInstructionNode),
    // I32Eqz(I32EqzInstructionNode),
    // I32Eq(I32EqInstructionNode),
    // I32LtS(I32LtSInstructionNode),
    // I32LtU(I32LtUInstructionNode),
    I32GeS(I32GeSInstructionNode),
    // I32GeU(I32GeUInstructionNode),
    // I32GtS(I32GtSInstructionNode),
    // I32GtU(I32GtUInstructionNode),
    // I64Add(I32AddInstructionNode),
    // I64Sub(I32SubInstructionNode),
    // Unreachable,
    // Nop,
    // BrTable(Vec<u32>, u32),
    // Return,
}

impl Node for InstructionNode {
    fn size(&self) -> u32 {
        match self {
            InstructionNode::Block(x) => x.size(),
            InstructionNode::Loop(x) => x.size(),
            InstructionNode::If(x) => x.size(),
            InstructionNode::Else(x) => x.size(),
            InstructionNode::Br(x) => x.size(),
            InstructionNode::BrIf(x) => x.size(),
            InstructionNode::Call(x) => x.size(),
            InstructionNode::End(x) => x.size(),
            InstructionNode::I32Const(x) => x.size(),
            InstructionNode::GetLocal(x) => x.size(),
            InstructionNode::SetLocal(x) => x.size(),
            InstructionNode::I32Add(x) => x.size(),
            // InstructionNode::I32Sub(x) => x.size(),
            // InstructionNode::I32RemU(x) => x.size(),
            // InstructionNode::I32Shl(x) => x.size(),
            // InstructionNode::I32Eqz(x) => x.size(),
            // InstructionNode::I32Eq(x) => x.size(),
            // InstructionNode::I32LtS(x) => x.size(),
            // InstructionNode::I32LtU(x) => x.size(),
            InstructionNode::I32GeS(x) => x.size(),
            InstructionNode::I32Sub(x) => x.size(),
            // InstructionNode::I32GeU(x) => x.size(),
            // InstructionNode::I32GtS(x) => x.size(),
            // InstructionNode::I32GtU(x) => x.size(),
            // InstructionNode::I64Add(x) => x.size(),
            // InstructionNode::I64Sub(x) => x.size(),
            // InstructionNode::Unreachable => 1,
            // InstructionNode::Nop => 1,
            // InstructionNode::BrTable(x, y) => 1 + encode_u32_to_leb128(x.len() as u32).len() as u32 + (x.len() as u32 * 4) + 4,
            // InstructionNode::Return => 1,
        }
    }

    fn encode(&self) -> Vec<u8> {
        match self {
            InstructionNode::Block(x) => x.encode(),
            InstructionNode::Loop(x) => x.encode(),
            InstructionNode::If(x) => x.encode(),
            InstructionNode::Else(x) => x.encode(),
            InstructionNode::Br(x) => x.encode(),
            InstructionNode::BrIf(x) => x.encode(),
            InstructionNode::Call(x) => x.encode(),
            InstructionNode::End(x) => x.encode(),
            InstructionNode::I32Const(x) => x.encode(),
            InstructionNode::GetLocal(x) => x.encode(),
            InstructionNode::SetLocal(x) => x.encode(),
            InstructionNode::I32Add(x) => x.encode(),
            // InstructionNode::I32Sub(x) => x.encode(),
            // InstructionNode::I32RemU(x) => x.encode(),
            // InstructionNode::I32Shl(x) => x.encode(),
            // InstructionNode::I32Eqz(x) => x.encode(),
            // InstructionNode::I32Eq(x) => x.encode(),
            // InstructionNode::I32LtS(x) => x.encode(),
            // InstructionNode::I32LtU(x) => x.encode(),
            InstructionNode::I32GeS(x) => x.encode(),
            InstructionNode::I32Sub(x) => x.encode(),
            // InstructionNode::I32GeU(x) => x.encode(),
            // InstructionNode::I32GtS(x) => x.encode(),
            // InstructionNode::I32GtU(x) => x.encode(),
            // InstructionNode::I64Add(x) => x.encode(),
            // InstructionNode::I64Sub(x) => x.encode(),
            // InstructionNode::Unreachable => vec![0x00],
            // InstructionNode::Nop => vec![0x01],
            // InstructionNode::BrTable(x, y) => {
            //     let mut buffer = vec![0x0e];
            //     buffer.extend(encode_u32_to_leb128(x.len() as u32));
            //     for i in x {
            //         buffer.extend(encode_u32_to_leb128(*i));
            //     }
            //     buffer.extend(encode_u32_to_leb128(*y));
            //     buffer
            // },
            // InstructionNode::Return => vec![0x0f],
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct I32ConstInstructionNode {
    pub opcode: u8,
    pub value: i32,
}

impl Node for I32ConstInstructionNode {
    fn size(&self) -> u32 {
        let mut size = 0;
        size += 1; // opcode
        size += encode_i32_to_leb128(self.value).len() as u32;
        size
    }

    fn encode(&self) -> Vec<u8> {
        let mut buffer = vec![];
        buffer.push(self.opcode);
        buffer.extend(encode_i32_to_leb128(self.value));
        buffer
    }
}

#[derive(Debug, Clone, Copy)]
pub struct EndInstructionNode {
    pub opcode: u8,
}

impl Node for EndInstructionNode {
    fn size(&self) -> u32 {
        1
    }

    fn encode(&self) -> Vec<u8> {
        vec![self.opcode]
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GetLocalInstructionNode {
    pub opcode: u8,
    pub index: u32,
}

impl Node for GetLocalInstructionNode {
    fn size(&self) -> u32 {
        let mut size = 0;
        size += 1; // opcode
        size += encode_u32_to_leb128(self.index).len() as u32;
        size
    }

    fn encode(&self) -> Vec<u8> {
        let mut buffer = vec![];
        buffer.push(self.opcode);
        buffer.extend(encode_u32_to_leb128(self.index));
        buffer
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SetLocalInstructionNode {
    pub opcode: u8,
    pub index: u32,
}

impl Node for SetLocalInstructionNode {
    fn size(&self) -> u32 {
        let mut size = 0;
        size += 1; // opcode
        size += encode_u32_to_leb128(self.index).len() as u32;
        size
    }

    fn encode(&self) -> Vec<u8> {
        let mut buffer = vec![];
        buffer.push(self.opcode);
        buffer.extend(encode_u32_to_leb128(self.index));
        buffer
    }
}

#[derive(Debug, Clone, Copy)]
pub struct I32AddInstructionNode {}

impl Node for I32AddInstructionNode {
    fn size(&self) -> u32 {
        1
    }

    fn encode(&self) -> Vec<u8> {
        vec![self.opcode()]
    }
}

impl I32AddInstructionNode {
    pub fn opcode(&self) -> u8 {
        0x6a
    }
}

#[derive(Debug, Clone, Copy)]
pub struct I32SubInstructionNode {}

impl Node for I32SubInstructionNode {
    fn size(&self) -> u32 {
        1
    }

    fn encode(&self) -> Vec<u8> {
        vec![self.opcode()]
    }
}

impl I32SubInstructionNode {
    pub fn opcode(&self) -> u8 {
        0x6b
    }
}

#[derive(Debug, Clone, Copy)]
pub struct I64AddInstructionNode {
    pub opcode: u8,
}

impl Node for I64AddInstructionNode {
    fn size(&self) -> u32 {
        1
    }

    fn encode(&self) -> Vec<u8> {
        vec![self.opcode]
    }
}

#[derive(Debug, Clone, Copy)]
pub struct I64SubInstructionNode {
    pub opcode: u8,
}

impl Node for I64SubInstructionNode {
    fn size(&self) -> u32 {
        1
    }

    fn encode(&self) -> Vec<u8> {
        vec![self.opcode]
    }
}

#[derive(Debug, Clone, Copy)]
pub struct I32RemUInstructionNode {
    pub opcode: u8,
}

impl Node for I32RemUInstructionNode {
    fn size(&self) -> u32 {
        1
    }

    fn encode(&self) -> Vec<u8> {
        vec![self.opcode]
    }
}

#[derive(Debug, Clone, Copy)]
pub struct I32ShlInstructionNode {
    pub opcode: u8,
}

impl Node for I32ShlInstructionNode {
    fn size(&self) -> u32 {
        1
    }

    fn encode(&self) -> Vec<u8> {
        vec![self.opcode]
    }
}

#[derive(Debug, Clone, Copy)]
pub struct I32EqzInstructionNode {
    pub opcode: u8,
}

impl Node for I32EqzInstructionNode {
    fn size(&self) -> u32 {
        1
    }

    fn encode(&self) -> Vec<u8> {
        vec![self.opcode]
    }
}

#[derive(Debug, Clone, Copy)]
pub struct I32EqInstructionNode {
    pub opcode: u8,
}

impl Node for I32EqInstructionNode {
    fn size(&self) -> u32 {
        1
    }

    fn encode(&self) -> Vec<u8> {
        vec![self.opcode]
    }
}

#[derive(Debug, Clone, Copy)]
pub struct I32LtSInstructionNode {
    pub opcode: u8,
}

impl Node for I32LtSInstructionNode {
    fn size(&self) -> u32 {
        1
    }

    fn encode(&self) -> Vec<u8> {
        vec![self.opcode]
    }
}

#[derive(Debug, Clone, Copy)]
pub struct I32LtUInstructionNode {
    pub opcode: u8,
}

impl Node for I32LtUInstructionNode {
    fn size(&self) -> u32 {
        1
    }

    fn encode(&self) -> Vec<u8> {
        vec![self.opcode]
    }
}

#[derive(Debug, Clone, Copy)]
pub struct I32GeSInstructionNode {
    pub opcode: u8,
}

impl Node for I32GeSInstructionNode {
    fn size(&self) -> u32 {
        1
    }

    fn encode(&self) -> Vec<u8> {
        vec![self.opcode]
    }
}

#[derive(Debug, Clone, Copy)]
pub struct I32GeUInstructionNode {
    pub opcode: u8,
}

impl Node for I32GeUInstructionNode {
    fn size(&self) -> u32 {
        1
    }

    fn encode(&self) -> Vec<u8> {
        vec![self.opcode]
    }
}

#[derive(Debug, Clone, Copy)]
pub struct I32GtSInstructionNode {
    pub opcode: u8,
}

impl Node for I32GtSInstructionNode {
    fn size(&self) -> u32 {
        1
    }

    fn encode(&self) -> Vec<u8> {
        vec![self.opcode]
    }
}

#[derive(Debug, Clone, Copy)]
pub struct I32GtUInstructionNode {
    pub opcode: u8,
}

impl Node for I32GtUInstructionNode {
    fn size(&self) -> u32 {
        1
    }

    fn encode(&self) -> Vec<u8> {
        vec![self.opcode]
    }
}

#[derive(Debug, Clone)]
pub struct IfInstructionNode {
    pub opcode: u8,
    pub block_type: BlockTypeNode,
    pub then_expr: ExpressionNode,
    pub else_expr: Option<ExpressionNode>,
}

impl Node for IfInstructionNode {
    fn size(&self) -> u32 {
        let mut size = 0;
        size += 1; // opcode
        size += self.block_type.size();
        size += self.then_expr.size();
        if let Some(else_expr) = &self.else_expr {
            size += else_expr.size();
        }
        size
    }

    fn encode(&self) -> Vec<u8> {
        let mut buffer = vec![];
        buffer.push(self.opcode);
        buffer.extend(self.block_type.encode());
        buffer.extend(self.then_expr.encode());
        if let Some(else_expr) = &self.else_expr {
            buffer.extend(else_expr.encode());
        }
        buffer
    }
}

#[derive(Debug, Clone)]
pub struct ElseInstructionNode {
    pub opcode: u8,
}

impl Node for ElseInstructionNode {
    fn size(&self) -> u32 {
        1
    }

    fn encode(&self) -> Vec<u8> {
        vec![self.opcode]
    }
}

#[derive(Debug, Clone)]
pub struct BlockInstructionNode {
    pub opcode: u8,
    pub block_type: BlockTypeNode,
    pub expr: ExpressionNode,
}

impl Node for BlockInstructionNode {
    fn size(&self) -> u32 {
        let mut size = 0;
        size += 1; // opcode
        size += self.block_type.size();
        size += self.expr.size();
        size
    }

    fn encode(&self) -> Vec<u8> {
        let mut buffer = vec![];
        buffer.push(self.opcode);
        buffer.extend(self.block_type.encode());
        buffer.extend(self.expr.encode());
        buffer
    }
}

#[derive(Debug, Clone)]
pub struct LoopInstructionNode {
    pub opcode: u8,
    pub block_type: BlockTypeNode,
    pub expr: ExpressionNode,
}

impl Node for LoopInstructionNode {
    fn size(&self) -> u32 {
        let mut size = 0;
        size += 1; // opcode
        size += self.block_type.size();
        size += self.expr.size();
        size
    }

    fn encode(&self) -> Vec<u8> {
        let mut buffer = vec![];
        buffer.push(self.opcode);
        buffer.extend(self.block_type.encode());
        buffer.extend(self.expr.encode());
        buffer
    }
}

#[derive(Debug, Clone)]
pub struct BrInstructionNode {
    pub opcode: u8,
    pub depth: u32,
}

impl Node for BrInstructionNode {
    fn size(&self) -> u32 {
        let mut size = 0;
        size += 1; // opcode
        size += encode_u32_to_leb128(self.depth).len() as u32;
        size
    }

    fn encode(&self) -> Vec<u8> {
        let mut buffer = vec![];
        buffer.push(self.opcode);
        buffer.extend(encode_u32_to_leb128(self.depth));
        buffer
    }
}

#[derive(Debug, Clone)]
pub struct BrIfInstructionNode {
    pub opcode: u8,
    pub depth: u32,
}

impl Node for BrIfInstructionNode {
    fn size(&self) -> u32 {
        let mut size = 0;
        size += 1; // opcode
        size += encode_u32_to_leb128(self.depth).len() as u32;
        size
    }

    fn encode(&self) -> Vec<u8> {
        let mut buffer = vec![];
        buffer.push(self.opcode);
        buffer.extend(encode_u32_to_leb128(self.depth));
        buffer
    }
}

#[derive(Debug, Clone)]
pub struct CallInstructionNode {
    pub opcode: u8,
    pub function_index: u32,
}

impl Node for CallInstructionNode {
    fn size(&self) -> u32 {
        let mut size = 0;
        size += 1; // opcode
        size += encode_u32_to_leb128(self.function_index).len() as u32;
        size
    }

    fn encode(&self) -> Vec<u8> {
        let mut buffer = vec![];
        buffer.push(self.opcode);
        buffer.extend(encode_u32_to_leb128(self.function_index));
        buffer
    }
}
