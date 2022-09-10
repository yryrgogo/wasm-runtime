use crate::node::Node;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum NumberTypeNode {
    I32,
    I64,
    F32,
    F64,
}

// https://doc.rust-lang.org/std/convert/trait.From.html
impl From<u8> for NumberTypeNode {
    fn from(byte: u8) -> Self {
        use NumberTypeNode::*;

        match byte {
            0x7F => I32,
            0x7E => I64,
            0x7D => F32,
            0x7C => F64,
            _ => unreachable!("Invalid ValueType {:x}", byte),
        }
    }
}

impl Into<u8> for NumberTypeNode {
    fn into(self) -> u8 {
        use NumberTypeNode::*;

        match self {
            I32 => 0x7F,
            I64 => 0x7E,
            F32 => 0x7D,
            F64 => 0x7C,
        }
    }
}

impl Node for NumberTypeNode {
    fn size(&self) -> u32 {
        1
    }

    fn encode(&self) -> Vec<u8> {
        use NumberTypeNode::*;
        match self {
            I32 => vec![0x7F],
            I64 => vec![0x7E],
            F32 => vec![0x7D],
            F64 => vec![0x7C],
        }
    }
}

#[derive(Debug)]
pub enum VectorTypeNode {
    V128,
}

impl From<u8> for VectorTypeNode {
    fn from(byte: u8) -> Self {
        use VectorTypeNode::*;

        match byte {
            0x7F => V128,
            _ => unreachable!("Invalid ValueType {:x}", byte),
        }
    }
}

pub enum ReferenceTypeNode {
    FunctionRef,
    ExternRef,
}

impl From<u8> for ReferenceTypeNode {
    fn from(byte: u8) -> Self {
        use ReferenceTypeNode::*;

        match byte {
            0x70 => FunctionRef,
            0x6F => ExternRef,
            _ => unreachable!("Invalid ReferenceType {:x}", byte),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ValueTypeNode {
    NumberType(NumberTypeNode),
    // VectorType(VectorType),
    // ReferenceType(ReferenceType),
}

impl From<u8> for ValueTypeNode {
    fn from(byte: u8) -> Self {
        use ValueTypeNode::*;

        match byte {
            0x7F | 0x7E | 0x7D | 0x7C => NumberType(NumberTypeNode::from(byte)),
            // 0x => VectorType(VectorType::from(byte)),
            // 0x | 0x6F => ReferenceType(ReferenceType::from(byte)),
            _ => unreachable!("Invalid ValueType {:x}", byte),
        }
    }
}

impl Into<u8> for ValueTypeNode {
    fn into(self) -> u8 {
        use ValueTypeNode::*;

        match self {
            NumberType(number_type) => number_type.into(),
            // VectorType(vector_type) => vector_type.into(),
            // ReferenceType(reference_type) => reference_type.into(),
        }
    }
}

impl Node for ValueTypeNode {
    fn size(&self) -> u32 {
        1
    }

    fn encode(&self) -> Vec<u8> {
        use ValueTypeNode::*;

        match self {
            NumberType(number_type) => number_type.encode(),
            // VectorType(vector_type) => vector_type.encode(),
            // ReferenceType(reference_type) => reference_type.encode(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BlockTypeNode {
    Empty,
    ValType(ValueTypeNode),
    // S33,
}

impl From<u8> for BlockTypeNode {
    fn from(x: u8) -> BlockTypeNode {
        match x {
            0x40 => BlockTypeNode::Empty,
            0x7F => BlockTypeNode::ValType(ValueTypeNode::NumberType(NumberTypeNode::I32)),
            // 0x7E => BlockType::ValType(ValueType::NumberType(NumberType::I64)),
            // 0x7D => BlockType::ValType(ValueType::NumberType(NumberType::F32)),
            // 0x7C => BlockType::ValType(ValueType::NumberType(NumberType::F64)),
            // 0x70 => BlockType::S33,
            _ => unreachable!("{} is an invalid value in BlockType", x),
        }
    }
}

impl Node for BlockTypeNode {
    fn size(&self) -> u32 {
        1
    }

    fn encode(&self) -> Vec<u8> {
        use BlockTypeNode::*;

        match self {
            Empty => vec![0x40],
            ValType(val_type) => val_type.encode(),
            // S33 => vec![0x70],
        }
    }
}
