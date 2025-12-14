pub const BUILT_IN_COMMANDS: [&str; 4] = ["type", "echo", "exit", "pwd"];

pub fn check_if_builtin_command(command: &str) -> bool {
    BUILT_IN_COMMANDS.contains(&command)
}

pub fn check_input_for_command(input: &str, command: &str) -> bool {
    input.trim().starts_with(command)
}

pub fn get_command_and_args(input: &str) -> (&str, Vec<&str>) {
    let parts: Vec<&str> = input.split_whitespace().collect();
    let (command, args) = parts.split_first().unwrap();

    (command, args.to_vec())
}

pub fn get_args_from_command(command: &str) -> Vec<&str> {
    let (_, args) = get_command_and_args(command);
    args
}
