mod decoder;
mod evaluator;
mod export;
mod import;
mod instructions;
mod module;
mod reader;
mod stack;
mod structure;
mod util;
use std::error::Error;
use util::args::get_args;

use evaluator::Evaluator;

use crate::decoder::Decoder;

fn main() -> Result<(), Box<dyn Error>> {
    let (path, num_args) = get_args();
    println!(
        "[wasm module path/args] path: {} args: {:?}",
        path, num_args
    );

    let mut decoder = Decoder::new(Some(&path), None).unwrap();

    // wasm binary の bytecode を確認する
    // let mut cnt = 0;
    // loop {
    //     let byte = decoder.reader.read_next_byte().unwrap();
    //     println!("{:03}: {:x}", cnt, byte);
    //     cnt += 1;
    // }

    decoder.run();

    let mut eval = Evaluator::new();
    for func_name in decoder.module.exports.keys() {
        if let Some(result) = eval.invoke(&decoder.module, func_name, num_args.clone()) {
            println!("{:#?}", result);
        } else {
            println!("{} end.", func_name);
        }
    }
    Ok(())
}
