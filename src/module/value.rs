#[derive(Debug)]
pub enum ValueType {
    Int32,
    Int64,
    Float32,
    Float64,
}
impl ValueType {
    pub fn from_byte(byte: u8) -> Option<ValueType> {
        match byte {
            0x7F => Some(ValueType::Int32),
            0x7E => Some(ValueType::Int64),
            0x7D => Some(ValueType::Float32),
            0x7C => Some(ValueType::Float64),
            _ => panic!("Invalid ValueType {:x}", byte),
        }
    }
}

#[derive(Debug)]
pub struct Value {
    bits: u8,
    value_type: ValueType,
}
impl Value {
    fn new(bits: u8, value_type: ValueType) -> Value {
        Value {
            bits: bits,
            value_type: value_type,
        }
    }

    pub fn i32() -> Value {
        Value::new(32, ValueType::Int32)
    }

    pub fn i64() -> Value {
        Value::new(64, ValueType::Int64)
    }

    pub fn f32() -> Value {
        Value::new(32, ValueType::Float32)
    }

    pub fn f64() -> Value {
        Value::new(64, ValueType::Float64)
    }
}
