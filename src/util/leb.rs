pub fn read_unsigned_leb128(
    unsigned_leb128_vec: Vec<u8>,
    max_bits: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut value: u32 = 0;
    let mut shift: u32 = 0;

    for byte in unsigned_leb128_vec {
        value |= u32::from(byte & 0x7F) << shift;
        shift += 7;

        if (byte >> 7) & 1 != 1 {
            break;
        }
        if shift > max_bits {
            panic!("Invalid LEB128 encoding");
        }
    }
    println!("{}", value);

    Ok(())
}

pub fn get_unsigned_leb128(value: u64) -> [u8; 8] {
    let mut buf = [0; 8];

    {
        let mut writable = &mut buf[..];
        leb128::write::unsigned(&mut writable, value).expect("Should write number");
    }

    buf
}

pub fn read_signed_leb128(
    signed_leb128_vec: Vec<u8>,
    max_bits: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut value: i32 = 0;
    let mut shift: u32 = 0;

    for byte in signed_leb128_vec {
        value |= i32::from(byte & 0x7F) << shift;
        shift += 7;

        if (byte >> 7) & 1 != 1 {
            break;
        }
        if shift > max_bits {
            panic!("Invalid LEB128 encoding");
        }
    }
    if (value >> (shift - 1)) & 1 == 1 {
        value |= !0 << shift;
    }
    println!("{}", value);

    Ok(())
}

pub fn get_signed_leb128(value: i64) -> [u8; 8] {
    let mut buf = [0; 8];

    {
        let mut writable = &mut buf[..];
        leb128::write::signed(&mut writable, value).expect("Should write number");
    }

    buf
}
