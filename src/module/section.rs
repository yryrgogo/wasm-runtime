pub enum SectionId {
    CustomSectionId = 0,
    TypeSectionId = 1,
    FunctionSectionId = 3,
    ExportSectionId = 7,
    CodeSectionId = 10,
}

impl SectionId {
    pub fn from_usize(n: u8) -> Option<SectionId> {
        match n {
            0 => Some(SectionId::CustomSectionId),
            1 => Some(SectionId::TypeSectionId),
            3 => Some(SectionId::FunctionSectionId),
            7 => Some(SectionId::ExportSectionId),
            10 => Some(SectionId::CodeSectionId),
            _ => panic!("Error: Not implemented"),
        }
    }
}

pub struct TypeSection;
impl TypeSection {
    pub fn validate_header(header: u8) {
        const HEADER: u8 = 0x60;
        if header != HEADER {
            panic!("Invalid TypeSection header {}", header);
        }
    }
}
