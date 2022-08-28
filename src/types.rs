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

pub enum ValueType {
    NumberType,
}

// https://webassembly.github.io/spec/core/binary/types.html#function-types
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
pub struct ResultTypeNode {
    // TODO: replace to Value Types
    pub val_types: Vec<NumberType>,
}

pub struct NumberTypeNode {
    pub ty: NumberType,
}
