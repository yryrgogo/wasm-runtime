use super::{function_type::FunctionType, number::Number};

#[derive(Debug, Clone)]
pub struct Function {
    func_type: FunctionType,
    pub local_vars: Vec<Number>,
    pub expressions: Vec<u8>,
}
impl Function {
    pub fn new(func_type: FunctionType) -> Function {
        Function {
            func_type: func_type,
            local_vars: vec![],
            expressions: vec![],
        }
    }
    pub fn inspect(&self) -> String {
        format!(
            "#<Function func_type:{} locals=[{}] expression={}>",
            self.func_type.inspect(),
            self.local_vars
                .iter()
                .map(|x| x.inspect())
                .collect::<Vec<String>>()
                .join(", "),
            self.expressions.len()
        )
    }
}
