use crate::{
    leb128::encode_u32_to_leb128,
    node::{CodeNode, ExportNode, FunctionTypeNode, Node},
};

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

impl Section for FunctionSectionNode {
    fn id(&self) -> SectionId {
        SectionId::FunctionSectionId
    }
}

impl Node for FunctionSectionNode {
    fn size(&self) -> u32 {
        let mut size = 0;
        size += 1; // count of type_indexes
        size += (&self.type_indexes).len() as u32;
        size
    }

    fn encode(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.push(self.id() as u8);
        bytes.push(self.size() as u8);
        bytes.push(self.type_indexes.len() as u8);
        for type_index in &self.type_indexes {
            bytes.extend(encode_u32_to_leb128(*type_index));
        }
        bytes
    }
}

#[derive(Debug)]
pub struct ExportSectionNode {
    pub exports: Vec<ExportNode>,
}

impl Section for ExportSectionNode {
    fn id(&self) -> SectionId {
        SectionId::ExportSectionId
    }
}

impl Node for ExportSectionNode {
    fn size(&self) -> u32 {
        let mut size = 0;
        size += 1; // count of exports
        for export in &self.exports {
            size += export.size();
        }
        size
    }

    fn encode(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.push(self.id() as u8);
        bytes.push(self.size() as u8);
        bytes.push(self.exports.len() as u8);
        for export in &self.exports {
            bytes.extend(export.encode());
        }
        bytes
    }
}

#[derive(Debug)]
pub struct CodeSectionNode {
    pub bodies: Vec<CodeNode>,
}

impl Section for CodeSectionNode {
    fn id(&self) -> SectionId {
        SectionId::CodeSectionId
    }
}

impl Node for CodeSectionNode {
    fn size(&self) -> u32 {
        let mut size = 0;
        size += 1; // count of bodies
        for body in &self.bodies {
            size += body.size();
        }
        size
    }

    fn encode(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.push(self.id() as u8);
        bytes.push(self.size() as u8);
        bytes.push(self.bodies.len() as u8);
        for body in &self.bodies {
            bytes.extend(body.encode());
        }
        bytes
    }
}
