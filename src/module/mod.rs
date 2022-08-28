pub mod section;

use std::error::Error;

use self::section::{CodeSectionNode, FunctionSectionNode, TypeSectionNode};

pub struct ModuleNode {
    pub type_section: Option<TypeSectionNode>,
    pub function_section: Option<FunctionSectionNode>,
    pub code_section: Option<CodeSectionNode>,
}
impl ModuleNode {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            function_section: None,
            type_section: None,
            code_section: None,
        })
    }

    pub fn validate_magic(bytes: &Vec<u8>) -> bool {
        if *bytes == [0x00, 0x61, 0x73, 0x6D] {
            true
        } else {
            panic!("Invalid magic bytes")
        }
    }

    pub fn validate_version(bytes: &Vec<u8>) -> bool {
        if *bytes == vec![0x01, 0x00, 0x00, 0x00] {
            true
        } else {
            panic!("Unsupported version")
        }
    }
}
