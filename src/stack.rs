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
