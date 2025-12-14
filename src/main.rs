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
            cmd if cmd.starts_with("echo") => {
                // this will print everything after "echo "
                let args = cmd[4..].trim();
                println!("{}", args);
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
