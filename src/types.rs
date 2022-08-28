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
