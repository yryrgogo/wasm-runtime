pub mod function;
pub mod section;
pub mod value;

pub struct Module {
    magic_bytes: String,
    version: u8,
}

impl Default for Module {
    fn default() -> Self {
        Self {
            magic_bytes: "\x00\x61\x73\x6D".to_string(),
            version: 1,
        }
    }
}

impl Module {
    fn version_bytes(&self, v: Option<u8>) -> &str {
        let version_bytes = match v.unwrap_or(1) {
            1 => "\x01\x00\x00\x00",
            _ => panic!("Error: Not implemented"),
        };
        version_bytes
    }

    fn header(&self, v: Option<u8>) -> String {
        self.magic_bytes.clone() + self.version_bytes(v)
    }

    pub fn valid_header(&self, header_string: &String) -> bool {
        header_string == &(self.header(Some(self.version)))
    }
}
