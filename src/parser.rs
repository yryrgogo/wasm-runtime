use super::types::NumberType;
use crate::{
    module::{
        section::{FunctionSectionNode, SectionId, TypeSectionNode},
        ModuleNode,
    },
    types::{FunctionTypeNode, ResultTypeNode},
};
use std::error::Error;

pub const LEB128_MAX_BITS: u32 = 32;

pub struct Parser {}

impl Parser {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {})
    }

    pub fn parse(&self, bytes: &mut Vec<u8>) -> Result<ModuleNode, Box<dyn Error>> {
        let (magic, version) = self.module_header(bytes).expect("Invalid header");
        ModuleNode::validate_magic(&magic);
        ModuleNode::validate_version(&version);

        let mut module = ModuleNode::new().expect("Invalid module");

        if bytes.len() == 0 {
            return Ok(module);
        }

        while bytes.len() > 0 {
            self.section(bytes, &mut module)
                .expect("Failed to parse section");
        }

        Ok(module)
    }

    fn module_header(&self, bytes: &mut Vec<u8>) -> Result<(Vec<u8>, Vec<u8>), Box<dyn Error>> {
        let magic_bytes = bytes[0..4].to_vec();
        let version = bytes[4..8].to_vec();
        *bytes = bytes[8..].to_vec();
        Ok((magic_bytes, version))
    }

    fn section(&self, bytes: &mut Vec<u8>, module: &mut ModuleNode) -> Result<(), Box<dyn Error>> {
        let id = Parser::read_u8(bytes).expect("Failed to parse section id");
        let (size, _) = Parser::read_u32(bytes).expect("Failed to parse section size");
        let mut section_bytes = bytes[0..(size as usize)].to_vec();
        (*bytes).drain(0..(size as usize));

        match SectionId::from(id) {
            SectionId::CustomSectionId => todo!("Custom section"),
            SectionId::TypeSectionId => {
                let section = self
                    .type_section(&mut section_bytes)
                    .expect("Failed to parse type section");
                (*module).type_section = Some(section);
            }
            SectionId::ImportSectionId => todo!("import section"),
            SectionId::FunctionSectionId => {
                let section = self
                    .function_section(&mut section_bytes)
                    .expect("Failed to parse function section");
                (*module).function_section = Some(section);
            }
            SectionId::GlobalSectionId => todo!("global section"),
            SectionId::ExportSectionId => todo!("export section"),
            SectionId::StartSectionId => todo!("start section"),
            SectionId::CodeSectionId => todo!("code section"),
            SectionId::ElementSectionId => todo!("element section"),
            SectionId::DataSectionId => todo!("data section"),
        };
        Ok(())
    }

    /// type section = section1(vec((functype)*))
    fn type_section(&self, bytes: &mut Vec<u8>) -> Result<TypeSectionNode, Box<dyn Error>> {
        let mut function_types: Vec<FunctionTypeNode> = vec![];
        let (size, _) = Parser::read_u32(bytes).expect("Failed to parse vector size");

        for _ in 0..size {
            let function_type = self
                .function_type(bytes)
                .expect("Failed to parse function type");
            function_types.push(function_type);
        }

        Ok(TypeSectionNode { function_types })
    }

    /// function section = section3(vec((typeidx)*))
    fn function_section(&self, bytes: &mut Vec<u8>) -> Result<FunctionSectionNode, Box<dyn Error>> {
        let mut type_indexes: Vec<u32> = vec![];
        let (size, _) = Parser::read_u32(bytes).expect("Failed to parse vector size");

        for _ in 0..size {
            let (type_index, _) = Parser::read_u32(bytes).expect("Failed to parse type index");
            type_indexes.push(type_index);
        }

        Ok(FunctionSectionNode { type_indexes })
    }

    /// functype = 0x60 (result type) (result type)
    fn function_type(&self, bytes: &mut Vec<u8>) -> Result<FunctionTypeNode, Box<dyn Error>> {
        let header = Parser::read_u8(bytes).expect("Failed to parse function type header");
        FunctionTypeNode::validate_header(header);

        let params = self
            .result_types(bytes)
            .expect("Failed to parse value type");

        // returns
        let returns = self
            .result_types(bytes)
            .expect("Failed to parse value type");

        let function_type_node: FunctionTypeNode = FunctionTypeNode { params, returns };

        Ok(function_type_node)
    }

    /// result type = vec((value type)*)
    fn result_types(&self, bytes: &mut Vec<u8>) -> Result<ResultTypeNode, Box<dyn Error>> {
        let (count, _) = Parser::read_u32(bytes).expect("Failed to read count");

        let mut node = ResultTypeNode { val_types: vec![] };
        for _ in 0..count {
            let number_type = self
                .number_type(bytes)
                .expect("Failed to parse number type");
            node.val_types.push(number_type);
        }
        Ok(node)
    }

    fn number_type(&self, bytes: &mut Vec<u8>) -> Result<NumberType, Box<dyn Error>> {
        let byte = Parser::read_u8(bytes).expect("Failed to read number type id");
        Ok(NumberType::from(byte))
    }

    pub fn read_u8(bytes: &mut Vec<u8>) -> Result<u8, Box<dyn Error>> {
        let byte = bytes[0];
        (*bytes).drain(0..1);
        Ok(byte)
    }

    pub fn read_u32(bytes: &mut Vec<u8>) -> Result<(u32, u32), Box<dyn Error>> {
        let mut value: u32 = 0;
        let mut shift: u32 = 0;
        let mut byte_count: u32 = 0;

        loop {
            let byte = bytes[0];
            (*bytes).drain(0..1);
            value |= u32::from(byte & 0x7f) << shift;
            shift += 7;
            byte_count += 1;

            if ((byte >> 7) & 1) != 1 {
                break;
            }
            if shift > LEB128_MAX_BITS {
                panic!("unsigned LEB128 overflow");
            }
        }
        Ok((value, byte_count))
    }

    fn read_i32(bytes: &mut Vec<u8>) -> Result<(isize, u32), Box<dyn Error>> {
        let mut value: isize = 0;
        let mut shift: u32 = 0;
        let mut byte_count: u32 = 0;

        loop {
            let byte = bytes[0];
            (*bytes).drain(0..1);
            value |= isize::from(byte & 0x7F) << shift;
            shift += 7;
            byte_count += 1;

            if ((byte >> 7) & 1) != 1 {
                break;
            }
            if shift > LEB128_MAX_BITS {
                panic!("signed LEB128 overflow");
            }
        }
        if (value >> (shift - 1)) & 1 == 1 {
            value |= !0 << shift;
        }
        Ok((value, byte_count))
    }
}

#[cfg(test)]
mod leb128_tests {
    use super::*;

    #[test]
    fn read_u32_case1() {
        let mut bytes = vec![229, 142, 38, 0, 0, 0, 0, 0];
        let (value, size) = Parser::read_u32(&mut bytes).expect("Invalid u32");
        assert_eq!(value, 624485);
        assert_eq!(size, 3);
    }

    #[test]
    fn read_u32_case2() {
        let mut bytes = vec![0x80, 0x80, 0xC0, 0x00, 0x0B];
        let (value, size) = Parser::read_u32(&mut bytes).expect("Invalid u32");

        assert_eq!(value, 1048576);
        assert_eq!(size, 4);
    }

    #[test]
    fn test_read_i32() {
        let mut bytes = vec![127, 0, 0, 0, 0, 0, 0, 0];
        let (value, size) = Parser::read_i32(&mut bytes).expect("Invalid i32");

        assert_eq!(value, -1);
        assert_eq!(size, 1);
    }
}
