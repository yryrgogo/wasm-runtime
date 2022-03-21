use super::value::Value;

pub struct Function {
    pub parameters: Vec<Value>,
    pub results: Vec<Value>,
}
impl Default for Function {
    fn default() -> Function {
        Function {
            parameters: vec![],
            results: vec![],
        }
    }
}
