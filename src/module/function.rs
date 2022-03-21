use super::value::Value;

pub struct Function {
    pub parameter: Vec<Value>,
}
impl Default for Function {
    fn default() -> Function {
        Function { parameter: vec![] }
    }
}
