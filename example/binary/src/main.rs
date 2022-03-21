mod util;
use crate::util::leb::read_unsigned_leb128;

fn main() {
    match read_unsigned_leb128(1) {
        Ok(()) => println!("OK!"),
        Err(err) => println!("Error! {}", err),
    }
}
