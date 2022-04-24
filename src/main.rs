mod decoder;
mod evaluator;
mod export;
mod instructions;
mod module;
mod stack;
mod structure;
mod util;
use std::error::Error;
// use std::fs::File;
// use std::io::{BufReader, Read};

use evaluator::Evaluator;
use module::number::Number;

use crate::decoder::Decoder;
use crate::util::leb;

fn main() -> Result<(), Box<dyn Error>> {
    // // LEB128 Test
    // let bytes = leb::get_unsigned_leb128(624485);
    // match leb::read_unsigned_leb128(&bytes.to_vec()) {
    //     Ok((value, size)) => println!("OK! size: {} {}", value, size),
    //     Err(err) => println!("Error! {}", err),
    // }

    // let signed_leb_arr = leb::get_signed_leb128(-1);
    // println!("singed leb128: {:?}", signed_leb_arr);
    // // let signed_leb_arr = [0xC0, 0xBB, 0x78];
    // match leb::read_signed_leb128(&signed_leb_arr.to_vec()) {
    //     Ok((value, size)) => {
    //         println!("OK! {} {}", value, size);
    //     }
    //     Err(err) => println!("Error! {}", err),
    // }

    // Main

    let path = "src/wasm/fib.wasm";
    // show_binary_hex(path).unwrap();

    let mut decoder = Decoder::new(path).unwrap();
    decoder.validate_header();
    decoder.decode_section().unwrap();
    decoder.inspect();

    // println!("Rest binary");
    // for i in 0..40 {
    //     let mut buf = [0; 1];
    //     decoder.reader.read_exact(&mut buf).unwrap();
    //     println!("{:03}: {:x}", i, buf[0]);
    // }

    let mut eval = Evaluator::new(decoder.module);
    eval.invoke("fib".to_string(), vec![Number::i32(Some(10))]);

    Ok(())
}

// fn show_binary_hex(path: &str) -> Result<(), Box<dyn Error>> {
//     let reader = BufReader::new(File::open(path)?);
//     for (i, byte) in reader.bytes().enumerate() {
//         println!("{:03}: {:x}", i, byte.unwrap());
//     }
//     Ok(())
// }
