mod decoder;
mod evaluator;
mod export;
mod instructions;
mod module;
mod reader;
mod stack;
mod structure;
mod util;
use std::env;
use std::error::Error;

use evaluator::Evaluator;
use module::number::Number;

use crate::decoder::Decoder;

fn main() -> Result<(), Box<dyn Error>> {
    let dir_path = env::current_dir().unwrap();
    let args: Vec<String> = env::args().collect();
    let wasm_module_path = args.get(1).unwrap_or_else(|| {
        panic!("wasm モジュールへのパスを渡してください。（ルートディレクトリからの相対パス）")
    });
    let path = format!("{}/{}", dir_path.to_string_lossy(), wasm_module_path);
    println!("[wasm module path]: {}", path);

    let mut decoder = Decoder::new(Some(&path), None).unwrap();

    decoder.run();

    let mut eval = Evaluator::new(decoder.module);
    eval.invoke("fib".to_string(), vec![Number::i32(Some(10))]);
    Ok(())
}
