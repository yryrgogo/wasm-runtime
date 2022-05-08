#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Value {
    Uint32(u32),
    Uint64(u64),
    Int32(i32),
    Int64(i64),
    Float32(f32),
    Float64(f64),
}

impl Value {
    pub fn u32(&self) -> u32 {
        match self {
            Value::Uint32(v) => *v,
            _ => unreachable!(),
        }
    }

    pub fn u64(&self) -> u64 {
        match self {
            Value::Uint64(v) => *v,
            _ => unreachable!(),
        }
    }

    pub fn i32(&self) -> i32 {
        match self {
            Value::Int32(v) => *v,
            _ => unreachable!("[Value::i32] {:?}", self),
        }
    }

    pub fn i64(&self) -> i64 {
        match self {
            Value::Int64(v) => *v,
            _ => unreachable!(),
        }
    }

    pub fn f32(&self) -> f32 {
        let valid_digit = 1000000.0;
        match self {
            Value::Float32(v) => (*v * valid_digit).floor() / valid_digit,
            _ => unreachable!(),
        }
    }

    // pub fn f64(&self) -> f64 {
    //     match self {
    //         Value::Float64(v) => *v,
    //         _ => unreachable!(),
    //     }
    // }
}
