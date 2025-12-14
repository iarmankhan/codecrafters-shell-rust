use is_executable::IsExecutable;
use std::env;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;

fn check_input_for_command(input: &str, command: &str) -> bool {
    input.trim().starts_with(command)
}

fn get_command_and_args(input: &str) -> (&str, Vec<&str>) {
    let parts: Vec<&str> = input.split_whitespace().collect();
    let (command, args) = parts.split_first().unwrap();

    (command, args.to_vec())
}

fn get_args_from_command(command: &str) -> Vec<&str> {
    let (_, args) = get_command_and_args(command);
    args
}

const BUILT_IN_COMMANDS: [&str; 3] = ["type", "echo", "exit"];

fn check_if_builtin_command(command: &str) -> bool {
    BUILT_IN_COMMANDS.contains(&command)
}

fn get_command_path(command: &str) -> Option<PathBuf> {
    let path = env::var("PATH").unwrap_or_default();

    for dir in env::split_paths(&path) {
        let full_path = dir.join(command);

        if full_path.exists() && full_path.is_executable() {
            return Some(full_path);
        }
    }

    return None;
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

                if let Some(_) = get_command_path(command) {
                    println!("{} is /usr/bin/{}", command, command);
                } else {
                    println!("{} not found", command);
                }
            }
            cmd if check_input_for_command(&cmd, "echo") => {
                let args: String = get_args_from_command(&cmd).join(" ");

                println!("{}", args);
            }
            cmd => {
                let (command, args) = get_command_and_args(&cmd);

                let command_path = get_command_path(command);

                if let None = command_path {
                    println!("{}: command not found", command);
                    return;
                }

                let output = Command::new(command_path.unwrap()).args(args).output();

                match output {
                    Ok(output) => {
                        io::stdout().write_all(&output.stdout).unwrap();
                        io::stderr().write_all(&output.stderr).unwrap();
                    }
                    Err(error) => {
                        io::stderr()
                            .write_all(format!("error executing command: {}\n", error).as_bytes())
                            .unwrap();
                    }
                }
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
