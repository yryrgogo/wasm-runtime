#[derive(PartialEq, Debug, Clone)]
pub enum Value {
    Int32(i32),
    Int64(i64),
    Float32(f32),
    Float64(f64),
}
