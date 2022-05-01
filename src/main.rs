mod decoder;
mod evaluator;
mod export;
mod instructions;
mod module;
mod reader;
mod stack;
mod structure;
mod util;
use std::error::Error;

use evaluator::Evaluator;
use module::number::Number;

use crate::decoder::Decoder;

fn main() -> Result<(), Box<dyn Error>> {
    let path = "src/wasm/fib.wasm";
    let mut decoder = Decoder::new(Some(path), None).unwrap();

    decoder.run();
    decoder.inspect();

    let mut eval = Evaluator::new(decoder.module);
    eval.invoke("fib".to_string(), vec![Number::i32(Some(10))]);

    // println!("Rest binary");
    // for i in 0..119 {
    //     let byte = decoder.reader.read_next_byte().unwrap();
    //     println!("{:03}: {:x}", i, byte);
    // }

    Ok(())
}

// fn show_binary_hex(path: &str) -> Result<(), Box<dyn Error>> {
//     let reader = BufReader::new(File::open(path)?);
//     for (i, byte) in reader.bytes().enumerate() {
//         println!("{:03}: {:x}", i, byte.unwrap());
//     }
//     Ok(())
// }
