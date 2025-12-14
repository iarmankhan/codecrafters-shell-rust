use std::io::{self, Write};

fn check_input_for_command(input: &str, command: &str) -> bool {
    input.trim().starts_with(command)
}

fn get_args_from_command(command: &str) -> Vec<&str> {
    let parts: Vec<&str> = command.split_whitespace().collect();
    let (_, args) = parts.split_first().unwrap();

    args.to_vec()
}

const BUILT_IN_COMMANDS: [&str; 3] = ["type", "echo", "exit"];

fn shell() {
    print!("$ ");
    io::stdout().flush().unwrap();

    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => match input.trim() {
            "exit" => {
                std::process::exit(0);
            }
            cmd if check_input_for_command(&cmd, "type") => {
                let args = get_args_from_command(&cmd);

                if args.is_empty() {
                    println!("invalid usage: type <command>");
                }

                if args.len() > 1 {
                    println!("type: too many arguments");
                }

                for arg in args {
                    if BUILT_IN_COMMANDS.contains(&arg) {
                        println!("{} is a shell builtin", arg);
                    } else {
                        println!("{}: not found", arg);
                    }
                }
            }
            cmd if check_input_for_command(&cmd, "echo") => {
                let args = get_args_from_command(&cmd).join(" ");

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
