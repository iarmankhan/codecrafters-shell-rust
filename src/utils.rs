pub const BUILT_IN_COMMANDS: [&str; 4] = ["type", "echo", "exit", "pwd"];

pub fn check_if_builtin_command(command: &str) -> bool {
    BUILT_IN_COMMANDS.contains(&command)
}

fn parse_command_with_quotes(x: &str) -> (String, usize) {
    let trimmed = x.trim();

    let mut in_single_quotes = false;
    let mut in_double_quotes = false;
    let mut result = String::new();
    let mut command_end_index: usize = trimmed.len();

    for (i, c) in trimmed.chars().enumerate() {
        if c == '"' && !in_single_quotes {
            in_double_quotes = !in_double_quotes;
            // Don't add quotes to result, but continue processing
        } else if c == '\'' && !in_double_quotes {
            in_single_quotes = !in_single_quotes;
            // Don't add quotes to result, but continue processing
        } else if c.is_whitespace() && !in_single_quotes && !in_double_quotes {
            command_end_index = i;
            break;
        } else {
            result.push(c);
        }
    }

    (result, command_end_index)
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
        } else if c == '\\' && in_single_quotes {
            // Take the backslash literally
            current_word.push(c);

            // take the next character literally
            if let Some(next_char) = trimmed.chars().nth(i + 1) {
                current_word.push(next_char);
            }
            i += 2;
            continue;
        } else if c == '\\' && in_double_quotes {
            // In double quotes, only certain characters are escaped
            if let Some(next_char) = trimmed.chars().nth(i + 1) {
                match next_char {
                    '"' | '\\' => {
                        current_word.push(next_char);
                        i += 2;
                        continue;
                    }
                    _ => {
                        // Take the backslash literally
                        current_word.push(c);
                        i += 1;
                        continue;
                    }
                }
            } else {
                // Backslash at end of string, take literally
                current_word.push(c);
                i += 1;
                continue;
            }
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

pub fn get_command_and_args(input: &str) -> (String, Vec<String>) {
    let trimmed = input.trim();

    let (command, command_end_index) = parse_command_with_quotes(input);

    let raw_args: &str = trimmed[command_end_index..].trim();

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
