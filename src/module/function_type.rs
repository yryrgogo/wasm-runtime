use super::number::NumberType;

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionType {
    pub parameters: Vec<NumberType>,
    pub results: Vec<NumberType>,
}

impl Default for FunctionType {
    fn default() -> FunctionType {
        FunctionType {
            parameters: vec![],
            results: vec![],
        }
    }
}
