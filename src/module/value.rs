#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Value {
    Int32(i32),
    Int64(i64),
    Float32(f32),
    Float64(f64),
}

impl Value {
    pub fn i32(&self) -> i32 {
        match self {
            Value::Int32(v) => *v,
            _ => unreachable!(),
        }
    }

    pub fn i64(&self) -> i64 {
        match self {
            Value::Int64(v) => *v,
            _ => unreachable!(),
        }
    }

    pub fn f32(&self) -> f32 {
        match self {
            Value::Float32(v) => *v,
            _ => unreachable!(),
        }
    }

    pub fn f64(&self) -> f64 {
        match self {
            Value::Float64(v) => *v,
            _ => unreachable!(),
        }
    }
}
