mod utils;

use crate::utils::{change_working_directory, check_if_builtin_command, get_command_and_args};
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
        Ok(_) => {
            let (cmd, args) = get_command_and_args(&input.trim());

            match cmd.as_str() {
                "pwd" => match env::current_dir() {
                    Ok(path) => println!("{}", path.display()),
                    Err(e) => eprintln!("error getting current directory: {}", e),
                },
                "exit" => {
                    std::process::exit(0);
                }
                "type" => {
                    if args.is_empty() {
                        println!("invalid usage: type <command>");
                        return;
                    }

                    if args.len() > 1 {
                        println!("type: too many arguments");
                        return;
                    }

                    let command = args[0].clone();

                    if check_if_builtin_command(&command) {
                        println!("{} is a shell builtin", command);
                        return;
                    }

                    if let Some(path) = get_command_path(&command) {
                        println!("{} is {}", command, path.display());
                    } else {
                        println!("{} not found", command);
                    }
                }
                "echo" => {
                    println!("{}", args.join(" "));
                }
                "cd" => {
                    let home = env::home_dir().unwrap_or_else(|| PathBuf::from("/"));

                    let path = if args.is_empty() {
                        home.clone()
                    } else {
                        PathBuf::from(args[0].as_str())
                    };

                    // Handle special case for "~"
                    if path.display().to_string() == "~".to_string() {
                        change_working_directory(&home).unwrap_or_else(|e| {
                            println!("cd: {}: {}", home.display(), e);
                        });
                        return;
                    }

                    // Check if the path exists and is a directory
                    if !path.exists() || !path.is_dir() {
                        println!("cd: {}: No such file or directory", path.display());
                        return;
                    }

                    change_working_directory(&path).unwrap_or_else(|e| {
                        println!("cd: {}: {}", path.display(), e);
                    });
                }
                cmd => {
                    let command_path = get_command_path(cmd);

                    if let None = command_path {
                        println!("{}: command not found", cmd);
                        return;
                    }

                    let output = Command::new(cmd).args(args).output();

                    match output {
                        Ok(output) => {
                            io::stdout().write_all(&output.stdout).unwrap();
                            io::stderr().write_all(&output.stderr).unwrap();
                        }
                        Err(error) => {
                            io::stderr()
                                .write_all(
                                    format!("error executing command: {}\n", error).as_bytes(),
                                )
                                .unwrap();
                        }
                    }
                }
            }
        }
        Err(error) => println!("error: {error}"),
    }
}

fn main() {
    loop {
        shell();
    }
}
