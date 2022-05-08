use super::value::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumberType {
    Uint32,
    Uint64,
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
                println!("  Invalid ValueType {:x}", byte);
                Some(NumberType::Int32)
            }
        };
        num_type
    }

    pub fn inspect(&self) -> String {
        format!("{:?}", self)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Number {
    pub num_type: NumberType,
    pub value: Value,
}
impl Number {
    pub fn new(num_type: NumberType, value: Value) -> Number {
        Number {
            num_type: num_type,
            value: value,
        }
    }

    pub fn u32(value: Option<u32>) -> Number {
        let v = value.unwrap();
        Number::new(NumberType::Uint32, Value::Uint32(v))
    }

    pub fn u64(value: Option<u64>) -> Number {
        let v = value.unwrap();
        Number::new(NumberType::Uint64, Value::Uint64(v))
    }

    pub fn i32(value: Option<i32>) -> Number {
        let v = value.unwrap();
        Number::new(NumberType::Int32, Value::Int32(v))
    }

    pub fn i64(value: Option<i64>) -> Number {
        let v = value.unwrap();
        Number::new(NumberType::Int64, Value::Int64(v))
    }

    pub fn f32(value: Option<f32>) -> Number {
        let v = value.unwrap();
        Number::new(NumberType::Float32, Value::Float32(v))
    }

    pub fn f64(value: Option<f64>) -> Number {
        let v = value.unwrap();
        Number::new(NumberType::Float64, Value::Float64(v))
    }
}
