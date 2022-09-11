use crate::node::Node;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum NumberType {
    I32,
    I64,
    F32,
    F64,
}

// https://doc.rust-lang.org/std/convert/trait.From.html
impl From<u8> for NumberType {
    fn from(byte: u8) -> Self {
        use NumberType::*;

        match byte {
            0x7F => I32,
            0x7E => I64,
            0x7D => F32,
            0x7C => F64,
            _ => unreachable!("Invalid ValueType {:x}", byte),
        }
    }
}

impl Into<u8> for NumberType {
    fn into(self) -> u8 {
        use NumberType::*;

        match self {
            I32 => 0x7F,
            I64 => 0x7E,
            F32 => 0x7D,
            F64 => 0x7C,
        }
    }
}

impl Node for NumberType {
    fn size(&self) -> u32 {
        1
    }

    fn encode(&self) -> Vec<u8> {
        use NumberType::*;
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
pub enum ValueType {
    Number(NumberType),
    // Vector(VectorType),
    // Reference(ReferenceType),
}

impl From<u8> for ValueType {
    fn from(byte: u8) -> Self {
        use ValueType::*;

        match byte {
            0x7F | 0x7E | 0x7D | 0x7C => Number(NumberType::from(byte)),
            // 0x => Vector(VectorType::from(byte)),
            // 0x | 0x6F => Reference(ReferenceType::from(byte)),
            _ => unreachable!("Invalid ValueType {:x}", byte),
        }
    }
}

impl Into<u8> for ValueType {
    fn into(self) -> u8 {
        use ValueType::*;

        match self {
            Number(number_type) => number_type.into(),
            // Vector(vector_type) => vector_type.into(),
            // Reference(reference_type) => reference_type.into(),
        }
    }
}

impl Node for ValueType {
    fn size(&self) -> u32 {
        1
    }

    fn encode(&self) -> Vec<u8> {
        use ValueType::*;

        match self {
            Number(number_type) => number_type.encode(),
            // Vector(vector_type) => vector_type.encode(),
            // Reference(reference_type) => reference_type.encode(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BlockType {
    Empty,
    ValType(ValueType),
    // S33,
}

impl From<u8> for BlockType {
    fn from(x: u8) -> BlockType {
        match x {
            0x40 => BlockType::Empty,
            0x7F => BlockType::ValType(ValueType::Number(NumberType::I32)),
            // 0x7E => BlockType::ValType(ValueType::Number(NumberType::I64)),
            // 0x7D => BlockType::ValType(ValueType::Number(NumberType::F32)),
            // 0x7C => BlockType::ValType(ValueType::Number(NumberType::F64)),
            // 0x70 => BlockType::S33,
            _ => unreachable!("{} is an invalid value in BlockType", x),
        }
    }
}

impl Node for BlockType {
    fn size(&self) -> u32 {
        1
    }

    fn encode(&self) -> Vec<u8> {
        use BlockType::*;

        match self {
            Empty => vec![0x40],
            ValType(val_type) => val_type.encode(),
            // S33 => vec![0x70],
        }
    }
}
