pub mod section;

use std::error::Error;

use self::section::{CodeSectionNode, ExportSectionNode, FunctionSectionNode, TypeSectionNode};

#[derive(Debug)]
pub struct ModuleNode {
    pub type_section: Option<TypeSectionNode>,
    pub function_section: Option<FunctionSectionNode>,
    pub export_section: Option<ExportSectionNode>,
    pub code_section: Option<CodeSectionNode>,
}
impl ModuleNode {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            type_section: None,
            function_section: None,
            export_section: None,
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
