pub mod section;

use std::error::Error;

use crate::buffer::Buffer;

use self::section::{
    CodeSectionNode, ExportSectionNode, FunctionSectionNode, Node, TypeSectionNode,
};

#[derive(Debug)]
pub struct ModuleNode {
    pub magic: [u8; 4],
    pub version: [u8; 4],
    pub type_section: Option<TypeSectionNode>,
    pub function_section: Option<FunctionSectionNode>,
    pub export_section: Option<ExportSectionNode>,
    pub code_section: Option<CodeSectionNode>,
    pub buffer: Buffer,
}
impl ModuleNode {
    pub fn new(magic: [u8; 4], version: [u8; 4]) -> Result<Self, Box<dyn Error>> {
        ModuleNode::validate_magic(&magic);
        ModuleNode::validate_version(&version);
        Ok(Self {
            magic,
            version,
            type_section: None,
            function_section: None,
            export_section: None,
            code_section: None,
            buffer: Buffer::new(),
        })
    }

    pub fn validate_magic(bytes: &[u8; 4]) -> bool {
        if *bytes == [0x00, 0x61, 0x73, 0x6D] {
            true
        } else {
            panic!("Invalid magic bytes")
        }
    }

    pub fn validate_version(bytes: &[u8; 4]) -> bool {
        if *bytes == [0x01, 0x00, 0x00, 0x00] {
            true
        } else {
            panic!("Unsupported version")
        }
    }

    pub fn emit(&mut self) {
        self.buffer.write_bytes(self.magic.to_vec());
        self.buffer.write_bytes(self.version.to_vec());

        if let Some(type_section) = &self.type_section {
            self.buffer.write_bytes(type_section.encode());
        }
        if let Some(function_section) = &self.function_section {
            println!("function_section: {:?}", function_section);
        }
        if let Some(export_section) = &self.export_section {
            println!("export_section: {:?}", export_section);
        }
        if let Some(code_section) = &self.code_section {
            println!("code_section: {:?}", code_section);
        }
    }
}
