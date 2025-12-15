pub const BUILT_IN_COMMANDS: [&str; 4] = ["type", "echo", "exit", "pwd"];

pub fn check_if_builtin_command(command: &str) -> bool {
    BUILT_IN_COMMANDS.contains(&command)
}

fn parse_args_with_quotes(x: &str) -> Vec<String> {
    let trimmed = x.trim();

    let mut in_single_quotes = false;
    let mut in_double_quotes = false;
    let mut parts: Vec<String> = Vec::new();

    let mut current_word = String::new();

    let n = trimmed.len();
    let mut i = 0;

    // Go through each character
    while i < n {
        let c = trimmed.chars().nth(i).unwrap();

        // Handle escape character
        if c == '\\' && !in_double_quotes && !in_single_quotes {
            // Get the next character if any
            if let Some(next_char) = trimmed.chars().nth(i + 1) {
                current_word.push(next_char);
            }
            i += 2;
            continue;
        }

        // If character is a double quote, toggle in_double_quotes
        if c == '"' {
            in_double_quotes = !in_double_quotes;
            // Do not include the quote character itself
            i += 1;
            continue;
        }

        // If character is a single quote & not in double quotes, toggle in_single_quotes
        if c == '\'' && !in_double_quotes {
            in_single_quotes = !in_single_quotes;

            // Do not include the quote character itself
            i += 1;
            continue;
        } else if c == '\'' && in_double_quotes {
            // Inside double quotes, single quotes are literal
            current_word.push(c);
            i += 1;
            continue;
        }

        // If character is a space and not in single quotes or double quotes, split here
        if c.is_whitespace() && !in_single_quotes && !in_double_quotes {
            // Boundary outside quotes; flush if non-empty
            if !current_word.is_empty() {
                parts.push(std::mem::take(&mut current_word));
            }
        } else {
            // Append character to current word (inside quotes, whitespace is preserved)
            current_word.push(c);
        }
        i += 1;
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
        parse_args_with_quotes(raw_args)
    };

    (command, args)
}

pub fn change_working_directory(path: &std::path::Path) -> Result<(), std::io::Error> {
    std::env::set_current_dir(path)
}
