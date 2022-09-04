#[derive(Debug, PartialEq, Eq)]
pub struct Buffer {
    pub bytes: Vec<u8>,
}

impl Buffer {
    pub fn new() -> Self {
        Self { bytes: vec![] }
    }

    pub fn write_u8(&mut self, value: u8) {
        self.bytes.push(value);
    }

    pub fn write_bytes(&mut self, bytes: Vec<u8>) {
        self.bytes.extend(bytes);
    }
}
