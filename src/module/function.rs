use std::{collections::HashMap, fmt::Debug};

use super::{
    function_type::FunctionType,
    number::{Number, NumberType},
};

#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    pub func_type: FunctionType,
    pub local_vars: Vec<NumberType>,
    pub bytecodes: Vec<u8>,
    pub blocks: HashMap<usize, Block>,
    pub index: Option<usize>,
}

impl Default for Function {
    fn default() -> Function {
        let func_type = FunctionType::default();
        Function::new(func_type, None)
    }
}

// impl Debug for Function {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//         write!(f, "{:#?}", self)
//     }
// }

impl Function {
    pub fn new(func_type: FunctionType, index: Option<usize>) -> Function {
        Function {
            func_type: func_type,
            local_vars: vec![],
            bytecodes: vec![],
            blocks: HashMap::new(),
            index: index,
        }
    }
    pub fn create_local_variables(&self) -> Vec<Number> {
        self.local_vars
            .iter()
            .map(|x| match x {
                NumberType::Int32 => Number::Int32(0),
                NumberType::Int64 => Number::Int64(0),
                NumberType::Float32 => Number::Float32(0.0),
                NumberType::Float64 => Number::Float64(0.0),
                _ => unreachable!(),
            })
            .collect::<Vec<Number>>()
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
}
