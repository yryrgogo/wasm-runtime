use std::error::Error;

#[derive(Debug, Clone, Copy)]
pub enum NumberType {
    Int32,
    Int64,
    Float32,
    Float64,
}
impl NumberType {
    pub fn from_byte(byte: u8) -> Option<NumberType> {
        match byte {
            0x7F => Some(NumberType::Int32),
            0x7E => Some(NumberType::Int64),
            0x7D => Some(NumberType::Float32),
            0x7C => Some(NumberType::Float64),
            // _ => panic!("Invalid ValueType {:x}", byte),
            _ => {
                println!("Invalid ValueType {:x}", byte);
                Some(NumberType::Int32)
            }
        }
    }

    pub fn decode_type(byte: u8) -> Result<NumberType, Box<dyn Error>> {
        Ok(match NumberType::from_byte(byte).unwrap() {
            NumberType::Int32 => NumberType::Int32,
            NumberType::Int64 => NumberType::Int64,
            NumberType::Float32 => NumberType::Float32,
            NumberType::Float64 => NumberType::Float64,
        })
    }

    pub fn inspect(&self) -> String {
        format!("{:?}", self)
    }
}

#[derive(PartialEq)]
pub enum Value {
    Int32(i32),
    Int64(i64),
    Float32(f32),
    Float64(f64),
}
pub struct Number {
    bits: u8,
    pub num_type: NumberType,
    pub value: Value,
}
impl Number {
    fn new(bits: u8, num_type: NumberType, value: Value) -> Number {
        Number {
            bits: bits,
            num_type: num_type,
            value: value,
        }
    }

    pub fn i32(value: Option<Value>) -> Number {
        Number::new(32, NumberType::Int32, value.unwrap_or(Value::Int32(0)))
    }

    pub fn i64(value: Option<Value>) -> Number {
        Number::new(64, NumberType::Int64, value.unwrap_or(Value::Int64(0)))
    }

    pub fn f32(value: Option<Value>) -> Number {
        Number::new(
            32,
            NumberType::Float32,
            value.unwrap_or(Value::Float32(0.0)),
        )
    }

    pub fn f64(value: Option<Value>) -> Number {
        Number::new(
            64,
            NumberType::Float64,
            value.unwrap_or(Value::Float64(0.0)),
        )
    }

    pub fn inspect(&self) -> String {
        format!("{:?}", self.num_type)
    }
}
