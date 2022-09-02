use std::env;

mod module;
mod node;
mod instruction;
mod parser;
mod types;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("file path: {:?}", *file_path);

    let mut bytes = std::fs::read(file_path).expect("file not found");

    let parser = parser::Parser::new().unwrap();
    parser.parse(&mut bytes).expect("Failed to decode");
    println!("Successfully decoded {:?}", bytes);
}
