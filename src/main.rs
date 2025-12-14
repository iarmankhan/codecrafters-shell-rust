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
                // Split the command into parts to handle multiple spaces
                let parts: Vec<&str> = cmd.split_whitespace().collect();

                // Join the arguments back together with a single space, skipping the "echo" part
                let args = parts
                    .iter()
                    .skip(1)
                    .cloned()
                    .collect::<Vec<&str>>()
                    .join(" ");

                // Print the arguments
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
