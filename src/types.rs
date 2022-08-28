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

pub enum ValueType {
    NumberType(NumberType),
    // VectorType(VectorType),
    // ReferenceType(ReferenceType),
}
