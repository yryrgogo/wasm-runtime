use crate::{
    module::{function::Block, number::Number},
    structure::frame::Frame,
};

#[derive(Debug, Clone)]
pub enum Instructions {
    Frame(Frame),
    Number(Number),
    Block(Block),
}
