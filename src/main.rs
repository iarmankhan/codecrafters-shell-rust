mod utils;

use crate::utils::{
    check_if_builtin_command, check_input_for_command, get_args_from_command, get_command_and_args,
};
use is_executable::IsExecutable;
use std::env;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;

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
            "pwd" => match env::current_dir() {
                Ok(path) => println!("{}", path.display()),
                Err(e) => eprintln!("error getting current directory: {}", e),
            },
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

                if let Some(path) = get_command_path(command) {
                    println!("{} is {}", command, path.display());
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

                let output = Command::new(command).args(args).output();

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
