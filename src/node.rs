use crate::types::ValueType;

// https://webassembly.github.io/spec/core/binary/types.html#function-types
#[derive(Debug)]
pub struct FunctionTypeNode {
    pub params: ResultTypeNode,
    pub returns: ResultTypeNode,
}
impl FunctionTypeNode {
    pub fn validate_header(header: u8) {
        const HEADER: u8 = 0x60;
        if header != HEADER {
            panic!("Invalid TypeSection header {}", header);
        }
    }
}

// https://webassembly.github.io/spec/core/binary/types.html#result-types
#[derive(Debug)]
pub struct ResultTypeNode {
    // TODO: replace to Value Types
    pub val_types: Vec<ValueType>,
}

#[derive(Debug)]
pub struct CodeNode {
    pub function_body_size: u32,
    pub local_count: u32,
    pub locals: Vec<LocalEntryNode>,
    pub expr: Vec<u8>,
}

#[derive(Debug)]
pub struct LocalEntryNode {
    pub count: u32,
    pub val_type: ValueType,
}
