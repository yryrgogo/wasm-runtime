pub fn byte2string(bytes: Box<[u8]>) -> String {
    let converted: String = String::from_utf8(bytes.to_vec()).unwrap_or("".to_string());
    converted
}
