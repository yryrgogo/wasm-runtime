use std::error::Error;

pub const LEB128_MAX_BITS: u32 = 32;

pub fn decode_unsigned_leb128(bytes: &mut Vec<u8>) -> Result<(u32, u32), Box<dyn Error>> {
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

pub fn decode_signed_leb128(bytes: &mut Vec<u8>) -> Result<(i32, u32), Box<dyn Error>> {
    let mut value: i32 = 0;
    let mut shift: u32 = 0;
    let mut byte_count: u32 = 0;

    loop {
        let byte = bytes[0];
        (*bytes).drain(0..1);
        value |= i32::from(byte & 0x7F) << shift;
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

pub fn encode_u32_to_leb128(mut value: u32) -> Vec<u8> {
    // unsigned leb128
    let mut result: Vec<u8> = vec![];
    loop {
        let byte = value & 0b01111111;
        value >>= 7;
        if value == 0 {
            result.push(byte as u8);
            break;
        } else {
            result.push((byte | 0b10000000) as u8);
        }
    }
    result
}

pub fn encode_i32_to_leb128(mut value: i32) -> Vec<u8> {
    // signed leb128
    let mut result: Vec<u8> = vec![];
    loop {
        let byte = value & 0b01111111;
        value >>= 7;

        if (value == 0 && (byte & 0b01000000) == 0) || (value == -1 && (byte & 0b01000000) != 0) {
            result.push(byte as u8);
            break;
        } else {
            result.push((byte | 0b10000000) as u8);
        }
    }
    result
}
