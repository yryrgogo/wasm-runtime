pub fn read_unsigned_leb128(max_bits: i64) -> Result<(), Box<dyn std::error::Error>> {
    let mut value: u32 = 0;
    let mut shift: u32 = 0;

    for byte in get_unsigned_leb128(624485) {
        if byte == 0 {
            break;
        }
        value |= u32::from(byte & 0x7F) << shift;
        shift += 7;
    }
    println!("{}", value);

    Ok(())
}
