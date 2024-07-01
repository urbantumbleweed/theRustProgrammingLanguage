use std::io;
use std::io::Write;

fn main() {

    let number: i32 = 5;

    let result: &str = if number % 6 == 0 {
        "divisible by 6"
    } else if number % 2 == 0 {
        "divisible by 2"
    } else if number % 3 == 0 {
        "divisible by 3"
    } else {
        "not divisible by 6, 2, or 3"
    };

    match io::stdout().write_all(result.as_bytes()) {
        Ok(_) => println!("{}", result),
        Err(e) => println!("Error: {}", e),
    }
}
