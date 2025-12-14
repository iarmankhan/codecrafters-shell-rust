use std::io::{self, Write};

fn main() {
    print!("$ ");
    io::stdout().flush().unwrap();

    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("{}: command not found", input.trim())
        }
        Err(error) => println!("error: {error}"),
    }
}
