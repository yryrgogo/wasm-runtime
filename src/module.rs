use crate::export::ExportMap;
use std::fmt::{Debug, Formatter, Result};

use self::function::Function;
use self::function_type::FunctionType;
use std::collections::HashMap;

pub mod function;
pub mod function_type;
pub mod number;
pub mod opcode;
pub mod section;

pub struct Module {
    magic_bytes: String,
    version: u8,
    pub functions: Vec<Function>,
    pub function_types: Vec<FunctionType>,
    pub exported: HashMap<String, ExportMap>,
}

impl Default for Module {
    fn default() -> Self {
        Self {
            magic_bytes: "\x00\x61\x73\x6D".to_string(),
            version: 1,
            functions: vec![],
            function_types: vec![],
            exported: HashMap::new(),
        }
    }
}

impl Debug for Module {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "
functions: {:#?}
function_types: {:#?}
exported: {:#?}
",
            self.functions, self.function_types, self.exported
        )
    }
}

impl Module {
    fn version_bytes(&self, v: Option<u8>) -> &str {
        let version_bytes = match v.unwrap_or(1) {
            1 => "\x01\x00\x00\x00",
            _ => unimplemented!(),
        };
        version_bytes
    }

    fn header(&self, v: Option<u8>) -> String {
        format!("{}{}", self.magic_bytes, self.version_bytes(v))
    }

    pub fn valid_header(&self, header_string: &String) -> bool {
        header_string == &(self.header(Some(self.version)))
    }
}
