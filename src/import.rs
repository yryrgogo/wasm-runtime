use crate::module::function_type::FunctionType;

#[derive(Debug)]
pub struct ImportMap {
    pub index: usize,
    pub function_type: FunctionType,
}
