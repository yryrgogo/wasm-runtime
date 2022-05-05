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
use util::args::get_module_path;

use evaluator::Evaluator;
use module::number::Number;

use crate::decoder::Decoder;

fn main() -> Result<(), Box<dyn Error>> {
    let path = get_module_path();
    println!("[wasm module path]: {}", path);

    let mut decoder = Decoder::new(Some(&path), None).unwrap();

    decoder.run();

    let mut eval = Evaluator::new();
    for func_name in decoder.module.exported.keys() {
        if let Some(result) = eval.invoke(&decoder.module, func_name, vec![Number::i32(Some(10))]) {
            println!("{:#?}", result);
        } else {
            println!("{} end.", func_name);
        }
    }
    Ok(())
}
