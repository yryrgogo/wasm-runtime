use super::module::Module;
use std::error::Error;

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
        Ok(module)
    }

    fn decode_header(&self, bytes: &mut Vec<u8>) -> Result<(Vec<u8>, Vec<u8>), Box<dyn Error>> {
        let magic_bytes = bytes[0..4].to_vec();
        let version = bytes[4..8].to_vec();
        *bytes = bytes[8..].to_vec();
        Ok((magic_bytes, version))
    }
}
