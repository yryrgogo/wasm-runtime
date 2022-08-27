use crate::module::function::Function;

#[derive(Debug, Clone)]
pub struct ExportMap {
    pub index: usize,
    pub function: Function,
}
