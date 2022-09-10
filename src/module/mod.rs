pub mod section;

use std::error::Error;

use crate::{buffer::Buffer, node::Node};

use self::section::{CodeSectionNode, ExportSectionNode, FunctionSectionNode, TypeSectionNode};

#[derive(Debug)]
pub struct ModuleNode {
    magic: [u8; 4],
    version: [u8; 4],
    type_section: Option<TypeSectionNode>,
    function_section: Option<FunctionSectionNode>,
    export_section: Option<ExportSectionNode>,
    code_section: Option<CodeSectionNode>,
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

    pub fn type_section(&self) -> Option<&TypeSectionNode> {
        self.type_section.as_ref()
    }

    pub fn set_type_section(&mut self, type_section: TypeSectionNode) {
        self.type_section = Some(type_section);
    }

    pub fn function_section(&self) -> Option<&FunctionSectionNode> {
        self.function_section.as_ref()
    }

    pub fn set_function_section(&mut self, function_section: FunctionSectionNode) {
        self.function_section = Some(function_section);
    }

    pub fn export_section(&self) -> Option<&ExportSectionNode> {
        self.export_section.as_ref()
    }

    pub fn set_export_section(&mut self, export_section: ExportSectionNode) {
        self.export_section = Some(export_section);
    }

    pub fn code_section(&self) -> Option<&CodeSectionNode> {
        self.code_section.as_ref()
    }

    pub fn set_code_section(&mut self, code_section: CodeSectionNode) {
        self.code_section = Some(code_section);
    }

    pub fn emit(&mut self) {
        self.buffer.write_bytes(self.magic.to_vec());
        self.buffer.write_bytes(self.version.to_vec());

        if let Some(type_section) = &self.type_section {
            self.buffer.write_bytes(type_section.encode());
        }
        if let Some(function_section) = &self.function_section {
            self.buffer.write_bytes(function_section.encode());
        }
        if let Some(export_section) = &self.export_section {
            self.buffer.write_bytes(export_section.encode());
        }
        if let Some(code_section) = &self.code_section {
            self.buffer.write_bytes(code_section.encode());
        }
    }

    pub fn write_to_file(&self, path: &str) -> Result<(), Box<dyn Error>> {
        self.buffer.write_to_file(path);
        Ok(())
    }
}
