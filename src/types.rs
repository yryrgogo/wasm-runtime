#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug)]
pub enum VectorType {
    V128,
}

impl From<u8> for VectorType {
    fn from(byte: u8) -> Self {
        use VectorType::*;

        match byte {
            0x7F => V128,
            _ => unreachable!("Invalid ValueType {:x}", byte),
        }
    }
}

pub enum ReferenceType {
    FunctionRef,
    ExternRef,
}

impl From<u8> for ReferenceType {
    fn from(byte: u8) -> Self {
        use ReferenceType::*;

        match byte {
            0x70 => FunctionRef,
            0x6F => ExternRef,
            _ => unreachable!("Invalid ReferenceType {:x}", byte),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ValueType {
    NumberType(NumberType),
    // VectorType(VectorType),
    // ReferenceType(ReferenceType),
}

#[derive(Debug, PartialEq, Eq)]
pub enum BlockType {
    Empty,
    ValType(ValueType),
    // S33,
}

impl From<u8> for BlockType {
    fn from(x: u8) -> BlockType {
        match x {
            0x40 => BlockType::Empty,
            0x7F => BlockType::ValType(ValueType::NumberType(NumberType::I32)),
            // 0x7E => BlockType::ValType(ValueType::NumberType(NumberType::I64)),
            // 0x7D => BlockType::ValType(ValueType::NumberType(NumberType::F32)),
            // 0x7C => BlockType::ValType(ValueType::NumberType(NumberType::F64)),
            // 0x70 => BlockType::S33,
            _ => unreachable!("{} is an invalid value in BlockType", x),
        }
    }
}
