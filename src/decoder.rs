use super::types::NumberType;
use crate::module::{
    section::{FunctionTypeNode, ResultTypeNode, SectionId, TypeSectionNode},
    Module,
};
use std::error::Error;

pub const LEB128_MAX_BITS: usize = 32;

pub struct Decoder {}

impl Decoder {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {})
    }

    pub fn decode(&self, bytes: &mut Vec<u8>) -> Result<Module, Box<dyn Error>> {
        let (magic, version) = self.decode_header(bytes).expect("Invalid header");
        Module::validate_magic(&magic);
        Module::validate_version(&version);

        let module = Module::new().expect("Invalid module");

        if bytes.len() == 0 {
            return Ok(module);
        }

        while bytes.len() > 0 {
            self.decode_section(bytes)
                .expect("Failed to decode section");
        }

        Ok(module)
    }

    fn decode_header(&self, bytes: &mut Vec<u8>) -> Result<(Vec<u8>, Vec<u8>), Box<dyn Error>> {
        let magic_bytes = bytes[0..4].to_vec();
        let version = bytes[4..8].to_vec();
        *bytes = bytes[8..].to_vec();
        Ok((magic_bytes, version))
    }

    fn decode_section(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error>> {
        let id = Decoder::read_u8(bytes).expect("Failed to read section id");
        let (size, _) = Decoder::read_u32(bytes).expect("Failed to read section size");
        let mut section_bytes = bytes[0..size].to_vec();
        (*bytes).drain(0..size);

        match SectionId::from_u8(id) {
            SectionId::CustomSectionId => todo!(),
            SectionId::TypeSectionId => self
                .decode_type_section(&mut section_bytes)
                .expect("Failed to decode type section"),
            SectionId::ImportSectionId => todo!("import section"),
            SectionId::FunctionSectionId => todo!("function section"),
            SectionId::GlobalSectionId => todo!(),
            SectionId::ExportSectionId => todo!(),
            SectionId::StartSectionId => todo!(),
            SectionId::CodeSectionId => todo!(),
        };
        Ok(())
    }

    /// type section = section1(vec(func type))
    fn decode_type_section(&self, bytes: &mut Vec<u8>) -> Result<TypeSectionNode, Box<dyn Error>> {
        let mut vector: Vec<FunctionTypeNode> = vec![];
        let (size, _) = Decoder::read_u32(bytes).expect("Failed to read vector size");

        for _ in 0..size {
            let function_type = self
                .decode_function_type(bytes)
                .expect("Failed to decode function type");
            vector.push(function_type);
        }

        Ok(TypeSectionNode { vector })
    }

    fn decode_function_type(
        &self,
        bytes: &mut Vec<u8>,
    ) -> Result<FunctionTypeNode, Box<dyn Error>> {
        let header = Decoder::read_u8(bytes).expect("Failed to read function type header");
        FunctionTypeNode::validate_header(header);

        let params = self
            .decode_result_types(bytes)
            .expect("Failed to decode value type");

        // returns
        let returns = self
            .decode_result_types(bytes)
            .expect("Failed to decode value type");

        let function_type_node: FunctionTypeNode = FunctionTypeNode { params, returns };

        Ok(function_type_node)
    }

    fn decode_result_types(&self, bytes: &mut Vec<u8>) -> Result<ResultTypeNode, Box<dyn Error>> {
        let (count, _) = Decoder::read_u32(bytes).expect("Failed to read count");

        let mut node = ResultTypeNode { val_types: vec![] };
        for _ in 0..count {
            let number_type = self
                .decode_number_type(bytes)
                .expect("Failed to decode number type");
            node.val_types.push(number_type);
        }
        Ok(node)
    }

    fn decode_number_type(&self, bytes: &mut Vec<u8>) -> Result<NumberType, Box<dyn Error>> {
        let byte = Decoder::read_u8(bytes).expect("Failed to read number type id");
        Ok(NumberType::from(byte))
    }

    pub fn read_u8(bytes: &mut Vec<u8>) -> Result<u8, Box<dyn Error>> {
        let byte = bytes[0];
        (*bytes).drain(0..1);
        Ok(byte)
    }

    pub fn read_u32(bytes: &mut Vec<u8>) -> Result<(usize, usize), Box<dyn Error>> {
        let mut value: usize = 0;
        let mut shift: usize = 0;
        let mut byte_count: usize = 0;

        loop {
            let byte = bytes[0];
            (*bytes).drain(0..1);
            value |= usize::from(byte & 0x7f) << shift;
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

    fn read_i32(bytes: &mut Vec<u8>) -> Result<(isize, usize), Box<dyn Error>> {
        let mut value: isize = 0;
        let mut shift: usize = 0;
        let mut byte_count: usize = 0;

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
        let (value, size) = Decoder::read_u32(&mut bytes).expect("Invalid u32");
        assert_eq!(value, 624485);
        assert_eq!(size, 3);
    }

    #[test]
    fn read_u32_case2() {
        let mut bytes = vec![0x80, 0x80, 0xC0, 0x00, 0x0B];
        let (value, size) = Decoder::read_u32(&mut bytes).expect("Invalid u32");

        assert_eq!(value, 1048576);
        assert_eq!(size, 4);
    }

    #[test]
    fn test_read_i32() {
        let mut bytes = vec![127, 0, 0, 0, 0, 0, 0, 0];
        let (value, size) = Decoder::read_i32(&mut bytes).expect("Invalid i32");

        assert_eq!(value, -1);
        assert_eq!(size, 1);
    }
}
