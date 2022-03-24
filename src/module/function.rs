use super::function_type::FunctionType;

#[derive(Debug, Clone)]
pub struct Function {
    func_type: FunctionType,
}
impl Function {
    pub fn new(func_type: FunctionType) -> Function {
        Function {
            func_type: func_type,
        }
    }
    pub fn inspect(&self) -> String {
        format!("# type: {}", self.func_type.inspect())
    }
}
