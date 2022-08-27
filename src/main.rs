use std::env;

mod decoder;
mod module;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("file path: {:?}", *file_path);

    let mut bytes = std::fs::read(file_path).expect("file not found");

    let decoder = decoder::Decoder::new().unwrap();
    decoder.decode(&mut bytes).expect("Failed to decode");
    println!("Successfully decoded {:?}", bytes);
}
