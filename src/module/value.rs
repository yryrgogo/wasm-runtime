use std::ops::{Add, Sub};

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Value {
    Int32(i32),
    Int64(i64),
    Float32(f32),
    Float64(f64),
}

impl Add for Value {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        self + self
    }
}

impl Sub for Value {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self - self
    }
}
