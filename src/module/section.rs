use crate::node::{CodeNode, FunctionTypeNode};

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

#[derive(Debug)]
pub struct TypeSectionNode {
    pub function_types: Vec<FunctionTypeNode>,
}

#[derive(Debug)]
pub struct FunctionSectionNode {
    pub type_indexes: Vec<u32>,
}

#[derive(Debug)]
pub struct CodeSectionNode {
    pub bodies: Vec<CodeNode>,
}
