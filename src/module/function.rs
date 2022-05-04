use std::collections::HashMap;

use super::{
    function_type::FunctionType,
    number::{Number, NumberType},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub func_type: FunctionType,
    pub local_vars: Vec<NumberType>,
    pub bytecodes: Vec<u8>,
    pub blocks: HashMap<usize, Block>,
}

impl Default for Function {
    fn default() -> Function {
        let func_type = FunctionType::default();
        Function::new(func_type, None)
    }
}

impl Function {
    pub fn new(func_type: FunctionType, index: Option<usize>) -> Function {
        Function {
            func_type: func_type,
            local_vars: vec![],
            bytecodes: vec![],
            blocks: HashMap::new(),
        }
    }
    pub fn create_local_variables(&self) -> Vec<Number> {
        self.local_vars
            .iter()
            .map(|x| match x {
                NumberType::Int32 => Number::i32(Some(0)),
                NumberType::Int64 => Number::i64(Some(0)),
                NumberType::Float32 => Number::f32(Some(0.0)),
                NumberType::Float64 => Number::f64(Some(0.0)),
                _ => unreachable!(),
            })
            .collect::<Vec<Number>>()
    }
    pub fn inspect(&self) -> String {
        format!(
            "#<Function func_type:{} locals=[{}] bytecodes={}>",
            self.func_type.inspect(),
            self.local_vars
                .iter()
                .map(|x| x.inspect())
                .collect::<Vec<String>>()
                .join(", "),
            self.bytecodes.len()
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub instruction: u8,
    pub arity: Vec<NumberType>,
    pub start_idx: usize,
    pub end_idx: usize,
}

impl Block {
    pub fn new(
        instruction: u8,
        arity: Vec<NumberType>,
        start_idx: usize,
        end_idx: Option<usize>,
    ) -> Block {
        let end = end_idx.unwrap_or(start_idx);
        Block {
            instruction: instruction,
            arity: arity,
            start_idx: start_idx,
            end_idx: end,
        }
    }

    pub fn inspect(&self) -> String {
        format!(
            "#<Block arity={:?} instruction={}, start_idx={}, end_idx={}>",
            self.arity, self.instruction, self.start_idx, self.end_idx
        )
    }
}
