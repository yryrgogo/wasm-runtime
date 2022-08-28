use crate::types::NumberType;

pub enum SectionId {
    CustomSectionId = 0,
    TypeSectionId = 1,
    ImportSectionId = 2,
    FunctionSectionId = 3,
    GlobalSectionId = 6,
    ExportSectionId = 7,
    StartSectionId = 8,
    CodeSectionId = 10,
}

impl SectionId {
    pub fn from_u8(n: u8) -> SectionId {
        match n {
            0 => SectionId::CustomSectionId,
            1 => SectionId::TypeSectionId,
            2 => SectionId::ImportSectionId,
            3 => SectionId::FunctionSectionId,
            6 => SectionId::GlobalSectionId,
            7 => SectionId::ExportSectionId,
            8 => SectionId::StartSectionId,
            10 => SectionId::CodeSectionId,
            _ => todo!("{} is not supported", n),
        }
    }
}

pub struct TypeSectionNode {
    pub vector: Vec<FunctionTypeNode>,
}
impl TypeSectionNode {}

// https://webassembly.github.io/spec/core/binary/types.html#function-types
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
pub struct ResultTypeNode {
    // TODO: replace to Value Types
    pub val_types: Vec<NumberType>,
}
