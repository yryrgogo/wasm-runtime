use crate::node::{CodeNode, ExportNode, FunctionTypeNode};

pub enum SectionId {
    CustomSectionId = 0x0,
    TypeSectionId = 0x1,
    ImportSectionId = 0x2,
    FunctionSectionId = 0x3,
    GlobalSectionId = 0x6,
    ExportSectionId = 0x7,
    StartSectionId = 0x8,
    ElementSectionId = 0x9,
    CodeSectionId = 0xA,
    DataSectionId = 0xB,
}

impl From<u8> for SectionId {
    fn from(x: u8) -> SectionId {
        use self::SectionId::*;
        match x {
            0x0 => CustomSectionId,
            0x1 => TypeSectionId,
            0x2 => ImportSectionId,
            0x3 => FunctionSectionId,
            0x6 => GlobalSectionId,
            0x7 => ExportSectionId,
            0x8 => StartSectionId,
            0x9 => ElementSectionId,
            0xA => CodeSectionId,
            0xB => DataSectionId,
            _ => todo!("{} is not supported", x),
        }
    }
}

trait Section {
    fn id(&self) -> SectionId;
}

pub trait Node {
    fn size(&self) -> u32;
    fn encode(&self) -> Vec<u8>;
}

#[derive(Debug)]
pub struct TypeSectionNode {
    pub function_types: Vec<FunctionTypeNode>,
}

impl Section for TypeSectionNode {
    fn id(&self) -> SectionId {
        SectionId::TypeSectionId
    }
}

impl Node for TypeSectionNode {
    fn size(&self) -> u32 {
        let mut size = 0;
        size += 1; // count of function types
        for function_type in &self.function_types {
            size += function_type.size();
        }
        size
    }

    fn encode(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.push(self.id() as u8);
        bytes.push(self.size() as u8);
        bytes.push(self.function_types.len() as u8);
        for function_type in &self.function_types {
            bytes.extend(function_type.encode());
        }
        bytes
    }
}

#[derive(Debug)]
pub struct FunctionSectionNode {
    pub type_indexes: Vec<u32>,
}

#[derive(Debug)]
pub struct ExportSectionNode {
    pub exports: Vec<ExportNode>,
}

#[derive(Debug)]
pub struct CodeSectionNode {
    pub bodies: Vec<CodeNode>,
}
