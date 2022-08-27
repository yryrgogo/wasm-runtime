use std::ops::{Add, Rem, Shl};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumberType {
    // Uint32,
    // Uint64,
    Int32,
    Int64,
    Float32,
    Float64,
}
impl NumberType {
    pub fn decode_byte(byte: u8) -> Option<NumberType> {
        let num_type = match byte {
            0x7F => Some(NumberType::Int32),
            0x7E => Some(NumberType::Int64),
            0x7D => Some(NumberType::Float32),
            0x7C => Some(NumberType::Float64),
            // _ => panic!("Invalid ValueType {:x}", byte),
            _ => {
                panic!("  Invalid ValueType {:x}", byte);
                // Some(NumberType::Int32)
            }
        };
        num_type
    }

    pub fn inspect(&self) -> String {
        format!("{:?}", self)
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Number {
    Uint32(u32),
    Uint64(u64),
    Int32(i32),
    Int64(i64),
    // Float https://github.com/WebAssembly/design/blob/main/BinaryEncoding.md#constants-described-here
    Float32(u32),
    Float64(u64),
}

impl Add for Number {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Number::Uint32(v1) => {
                if let Number::Uint32(v2) = rhs {
                    Number::Uint32(v1 + v2)
                } else {
                    unreachable!()
                }
            }
            Number::Uint64(v1) => {
                if let Number::Uint64(v2) = rhs {
                    Number::Uint64(v1 + v2)
                } else {
                    unreachable!()
                }
            }
            Number::Int32(v1) => {
                if let Number::Int32(v2) = rhs {
                    Number::Int32(v1 + v2)
                } else {
                    unreachable!()
                }
            }
            Number::Int64(v1) => {
                if let Number::Int64(v2) = rhs {
                    Number::Int64(v1 + v2)
                } else {
                    unreachable!()
                }
            }
            Number::Float32(v1) => {
                if let Number::Float32(v2) = rhs {
                    Number::Float32(v1 + v2)
                } else {
                    unreachable!()
                }
            }
            Number::Float64(v1) => {
                if let Number::Float64(v2) = rhs {
                    Number::Float64(v1 + v2)
                } else {
                    unreachable!()
                }
            }
        }
    }
}

impl Rem for Number {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        match self {
            Number::Uint32(v1) => {
                if let Number::Uint32(v2) = rhs {
                    Number::Uint32(v1 % v2)
                } else {
                    unreachable!()
                }
            }
            Number::Uint64(v1) => {
                if let Number::Uint64(v2) = rhs {
                    Number::Uint64(v1 % v2)
                } else {
                    unreachable!()
                }
            }
            Number::Int32(v1) => {
                if let Number::Int32(v2) = rhs {
                    Number::Int32(v1 % v2)
                } else {
                    unreachable!()
                }
            }
            Number::Int64(v1) => {
                if let Number::Int64(v2) = rhs {
                    Number::Int64(v1 % v2)
                } else {
                    unreachable!()
                }
            }
            Number::Float32(v1) => {
                if let Number::Float32(v2) = rhs {
                    Number::Float32(v1 % v2)
                } else {
                    unreachable!()
                }
            }
            Number::Float64(v1) => {
                if let Number::Float64(v2) = rhs {
                    Number::Float64(v1 % v2)
                } else {
                    unreachable!()
                }
            }
        }
    }
}

impl Shl for Number {
    type Output = Self;
    fn shl(self, rhs: Self) -> Self::Output {
        match self {
            Number::Uint32(v1) => {
                if let Number::Uint32(v2) = rhs {
                    Number::Uint32(v1 << v2)
                } else {
                    unreachable!()
                }
            }
            Number::Uint64(v1) => {
                if let Number::Uint64(v2) = rhs {
                    Number::Uint64(v1 << v2)
                } else {
                    unreachable!()
                }
            }
            Number::Int32(v1) => {
                if let Number::Int32(v2) = rhs {
                    Number::Int32(v1 << v2)
                } else {
                    unreachable!()
                }
            }
            Number::Int64(v1) => {
                if let Number::Int64(v2) = rhs {
                    Number::Int64(v1 << v2)
                } else {
                    unreachable!()
                }
            }
            Number::Float32(v1) => {
                unreachable!()
            }
            Number::Float64(v1) => {
                unreachable!()
            }
        }
    }
}

// #[derive(Debug, PartialEq, Clone)]
// struct Uint32(u32);
// impl Uint32 {
//     pub fn value(&self) -> u32 {
//         self.0
//     }
// }

// #[derive(Debug, PartialEq, Clone)]
// struct Uint32(u32);
// impl Uint32 {
//     pub fn value(&self) -> u32 {
//         self.0
//     }
// }
