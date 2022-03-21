mod decoder;
mod module;
mod util;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};

use crate::decoder::Decoder;
use crate::util::leb;

fn main() -> Result<(), Box<dyn Error>> {
    // LEB128 Test
    let unsigned_leb_arr = leb::get_unsigned_leb128(624485);
    match leb::read_unsigned_leb128(unsigned_leb_arr.to_vec(), 32) {
        Ok(()) => println!("OK!"),
        Err(err) => println!("Error! {}", err),
    }

    let signed_leb_arr = leb::get_signed_leb128(-123456);
    // let signed_leb_arr = [0xC0, 0xBB, 0x78];
    match leb::read_signed_leb128(signed_leb_arr.to_vec(), 32) {
        Ok(()) => println!("OK!"),
        Err(err) => println!("Error! {}", err),
    }

    // Main

    let path = "src/wasm/fib.wasm";
    // show_binary_hex(path).unwrap();

    let mut decoder = Decoder::new(path).unwrap();
    decoder.validate_header();
    decoder.decode_section().unwrap();

    Ok(())
}

fn show_binary_hex(path: &str) -> Result<(), Box<dyn Error>> {
    let reader = BufReader::new(File::open(path)?);
    for byte in reader.bytes() {
        println!("{:x}", byte.unwrap());
    }
    Ok(())
}
