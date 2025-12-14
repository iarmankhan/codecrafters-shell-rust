use is_executable::IsExecutable;
use std::env;
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

fn check_if_builtin_command(command: &str) -> bool {
    BUILT_IN_COMMANDS.contains(&command)
}

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
                    return;
                }

                if args.len() > 1 {
                    println!("type: too many arguments");
                    return;
                }

                let command = args[0];

                if check_if_builtin_command(command) {
                    println!("{} is a shell builtin", command);
                    return;
                }

                let path = env::var("PATH").unwrap_or_default();

                for dir in env::split_paths(&path) {
                    let full_path = dir.join(command);

                    if full_path.exists() && full_path.is_executable() {
                        println!("{} is {}", command, full_path.display());
                        return;
                    }
                }

                println!("{}: not found", command);
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
