use crate::{module::number::Number, structure::frame::Frame};

#[derive(Debug)]
pub enum Instructions {
    Frame(Frame),
    Number(Number),
}
