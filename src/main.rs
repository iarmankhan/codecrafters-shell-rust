use std::io::{self, Write};

fn shell() {
    print!("$ ");
    io::stdout().flush().unwrap();

    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => match input.trim() {
            "exit" => {
                std::process::exit(0);
            }
            command => {
                println!("{}: command not found", command)
            }
        },
        Err(error) => println!("error: {error}"),
    }
}

fn main() {
    loop {
        shell();
    }
}
