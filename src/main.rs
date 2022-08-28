use std::env;

mod module;
mod parser;
mod types;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("file path: {:?}", *file_path);

    let mut bytes = std::fs::read(file_path).expect("file not found");

    let decoder = parser::Parser::new().unwrap();
    decoder.parse(&mut bytes).expect("Failed to decode");
    println!("Successfully decoded {:?}", bytes);
}
