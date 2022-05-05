use crate::module::function::Function;

#[derive(Debug)]
pub struct ExportMap {
    pub index: usize,
    pub function: Function,
}
