use crate::{module::value::Value, structure::frame::Frame};

#[derive(Debug)]
pub enum Instructions {
    Frame(Frame),
    Value(Value),
}
