const UNSIGNED_LEB128_MAX_BITS: usize = 32;

pub fn read_unsigned_leb128(bytes: &Vec<u8>) -> Result<(usize, usize), Box<dyn std::error::Error>> {
    let mut value: usize = 0;
    let mut shift: usize = 0;
    let mut size: usize = 0;

    for byte in bytes {
        value |= usize::from(byte & 0x7F) << shift;
        shift += 7;
        size += 1;

        if (byte >> 7) & 1 != 1 {
            break;
        }
        if shift > UNSIGNED_LEB128_MAX_BITS {
            panic!("Invalid LEB128 encoding");
        }
    }

    Ok((value, size))
}

pub fn get_unsigned_leb128(value: u64) -> [u8; 8] {
    let mut buf = [0; 8];

    {
        let mut writable = &mut buf[..];
        leb128::write::unsigned(&mut writable, value).expect("Should write number");
    }

    buf
}

pub fn read_signed_leb128(bytes: &Vec<u8>) -> Result<(isize, usize), Box<dyn std::error::Error>> {
    let mut value: isize = 0;
    let mut shift: usize = 0;
    let mut size: usize = 0;

    for byte in bytes {
        value |= isize::from(byte & 0x7F) << shift;
        shift += 7;
        size += 1;

        if (byte >> 7) & 1 != 1 {
            break;
        }
        if shift > UNSIGNED_LEB128_MAX_BITS {
            panic!("Invalid LEB128 encoding");
        }
    }
    if (value >> (shift - 1)) & 1 == 1 {
        value |= !0 << shift;
    }

    Ok((value, size))
}

pub fn get_signed_leb128(value: i64) -> [u8; 8] {
    let mut buf = [0; 8];

    {
        let mut writable = &mut buf[..];
        leb128::write::signed(&mut writable, value).expect("Should write number");
    }

    buf
}
