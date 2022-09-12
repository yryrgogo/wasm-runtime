use std::ops::Add;

#[derive(Debug, Clone)]
pub enum StackEntry {
    value(Value),
}

#[derive(Debug, Clone)]
pub enum Value {
    num(Number),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Number {
    i32(i32),
    i64(i64),
    f32(f32),
    f64(f64),
}

impl Add for Number {
    type Output = Number;

    fn add(self, other: Number) -> Number {
        match (self, other) {
            (Number::i32(a), Number::i32(b)) => Number::i32(a + b),
            (Number::i64(a), Number::i64(b)) => Number::i64(a + b),
            (Number::f32(a), Number::f32(b)) => Number::f32(a + b),
            (Number::f64(a), Number::f64(b)) => Number::f64(a + b),
            _ => panic!("Cannot add numbers of different types"),
        }
    }
}
