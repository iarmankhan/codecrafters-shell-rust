pub const BUILT_IN_COMMANDS: [&str; 4] = ["type", "echo", "exit", "pwd"];

pub fn check_if_builtin_command(command: &str) -> bool {
    BUILT_IN_COMMANDS.contains(&command)
}

fn parse_args_single_quote(x: &str) -> Vec<String> {
    let trimmed = x.trim();

    let mut in_single_quotes = false;
    let mut parts: Vec<String> = Vec::new();

    let mut current_word = String::new();

    // Go through each character
    for c in trimmed.chars() {
        // If character is a single quote, toggle in_single_quotes
        if c == '\'' {
            in_single_quotes = !in_single_quotes;

            // Do not include the quote character itself
            continue;
        }

        // If character is a space and not in single quotes, split here
        if c.is_whitespace() && !in_single_quotes {
            // Boundary outside quotes; flush if non-empty
            if !current_word.is_empty() {
                parts.push(std::mem::take(&mut current_word));
            }
        } else {
            // Append character to current word (inside quotes, whitespace is preserved)
            current_word.push(c);
        }
    }

    if !current_word.is_empty() {
        parts.push(current_word);
    }

    return parts;
}

pub fn get_command_and_args(input: &str) -> (&str, Vec<String>) {
    let trimmed = input.trim();

    // Find the end of first token
    let end = trimmed
        .find(char::is_whitespace)
        .unwrap_or_else(|| trimmed.len());

    let command = &trimmed[..end];

    let raw_args: &str = trimmed[end..].trim();

    // Parse args with single quote support
    let args = if raw_args.is_empty() {
        Vec::new()
    } else {
        parse_args_single_quote(raw_args)
    };

    (command, args)
}

pub fn change_working_directory(path: &std::path::Path) -> Result<(), std::io::Error> {
    std::env::set_current_dir(path)
}
