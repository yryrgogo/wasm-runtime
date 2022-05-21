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
    pub fn from_usize(n: u8) -> Option<SectionId> {
        match n {
            0 => Some(SectionId::CustomSectionId),
            1 => Some(SectionId::TypeSectionId),
            2 => Some(SectionId::ImportSectionId),
            3 => Some(SectionId::FunctionSectionId),
            6 => Some(SectionId::GlobalSectionId),
            7 => Some(SectionId::ExportSectionId),
            8 => Some(SectionId::StartSectionId),
            10 => Some(SectionId::CodeSectionId),
            _ => todo!("{}", n),
        }
    }
}

pub struct TypeSection;
impl TypeSection {
    pub fn validate_type_entry_header(header: u8) {
        const HEADER: u8 = 0x60;
        if header != HEADER {
            panic!("Invalid TypeSection header {}", header);
        }
    }
}

pub enum ExternalKind {
    Func = 0x00,
    Table = 0x01,
    LinearMemory = 0x02,
    GlobalVariable = 0x03,
}

impl ExternalKind {
    pub fn from_usize(n: u8) -> Option<ExternalKind> {
        match n {
            0 => Some(ExternalKind::Func),
            1 => Some(ExternalKind::Table),
            2 => Some(ExternalKind::LinearMemory),
            3 => Some(ExternalKind::GlobalVariable),
            _ => panic!("Invalid Export Kind {}", n),
        }
    }
}
