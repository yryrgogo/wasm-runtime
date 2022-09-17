use std::ops::{Add, Div, Mul, Rem, Sub};

use crate::types::BlockType;

#[derive(Debug, Clone)]
pub enum StackEntry {
    value(Value),
    label(Label),
}

#[derive(Debug, Clone)]
pub struct Label {
    pub label_type: LabelType,
    pub arity: BlockType,
    pub size: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LabelType {
    Block,
    Loop,
    If,
}

#[derive(Debug, Clone)]
pub enum Value {
    num(Number),
}

#[derive(Debug, Clone)]
pub enum Number {
    i32(i32),
    i64(i64),
    f32(f32),
    f64(f64),
}

impl Add for Number {
    type Output = Number;

    fn add(self, rhs: Number) -> Number {
        match (self, rhs) {
            (Number::i32(a), Number::i32(b)) => Number::i32(a + b),
            (Number::i64(a), Number::i64(b)) => Number::i64(a + b),
            (Number::f32(a), Number::f32(b)) => Number::f32(a + b),
            (Number::f64(a), Number::f64(b)) => Number::f64(a + b),
            _ => panic!("Cannot add numbers of different types"),
        }
    }
}

impl Sub for Number {
    type Output = Number;

    fn sub(self, rhs: Number) -> Number {
        match (self, rhs) {
            (Number::i32(a), Number::i32(b)) => Number::i32(a - b),
            (Number::i64(a), Number::i64(b)) => Number::i64(a - b),
            (Number::f32(a), Number::f32(b)) => Number::f32(a - b),
            (Number::f64(a), Number::f64(b)) => Number::f64(a - b),
            _ => panic!("Cannot subtract numbers of different types"),
        }
    }
}

impl Mul for Number {
    type Output = Number;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Number::i32(a), Number::i32(b)) => Number::i32(a * b),
            (Number::i64(a), Number::i64(b)) => Number::i64(a * b),
            (Number::f32(a), Number::f32(b)) => Number::f32(a * b),
            (Number::f64(a), Number::f64(b)) => Number::f64(a * b),
            _ => panic!("Cannot multiply numbers of different types"),
        }
    }
}

impl Div for Number {
    type Output = Number;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Number::i32(a), Number::i32(b)) => Number::i32(a / b),
            (Number::i64(a), Number::i64(b)) => Number::i64(a / b),
            (Number::f32(a), Number::f32(b)) => Number::f32(a / b),
            (Number::f64(a), Number::f64(b)) => Number::f64(a / b),
            _ => panic!("Cannot divide numbers of different types"),
        }
    }
}

impl Rem for Number {
    type Output = Number;

    fn rem(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Number::i32(a), Number::i32(b)) => Number::i32(a % b),
            (Number::i64(a), Number::i64(b)) => Number::i64(a % b),
            (Number::f32(a), Number::f32(b)) => Number::f32(a % b),
            (Number::f64(a), Number::f64(b)) => Number::f64(a % b),
            _ => panic!("Cannot divide numbers of different types"),
        }
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Number::i32(a), Number::i32(b)) => a == b,
            (Number::i64(a), Number::i64(b)) => a == b,
            (Number::f32(a), Number::f32(b)) => a == b,
            (Number::f64(a), Number::f64(b)) => a == b,
            _ => panic!("Cannot compare numbers of different types"),
        }
    }
}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Number::i32(a), Number::i32(b)) => a.partial_cmp(b),
            (Number::i64(a), Number::i64(b)) => a.partial_cmp(b),
            (Number::f32(a), Number::f32(b)) => a.partial_cmp(b),
            (Number::f64(a), Number::f64(b)) => a.partial_cmp(b),
            _ => panic!("Cannot compare numbers of different types"),
        }
    }
}
