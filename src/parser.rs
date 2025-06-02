use crate::error::ShellError;
use std::io::{Write, stdin, stdout};

#[derive(Debug, Clone, Copy, PartialEq)]
enum ParseState {
    Normal,
    SingleQuote,
    DoubleQuote,
}

pub fn parse_command(input: String) -> Result<(String, Vec<String>), ShellError> {
    let input = input.trim();

    if input.is_empty() {
        return Ok((String::new(), Vec::new()));
    }

    if input.contains('`') {
        return Err(ShellError::Backticks);
    }

    let parts = parser(input)?;

    if parts.is_empty() {
        return Ok((String::new(), Vec::new()));
    }

    let cmd = parts[0].to_string();
    let args = parts[1..].to_vec();

    Ok((cmd, args))
}

fn parser(input: &str) -> Result<Vec<String>, ShellError> {
    let mut full_input = input.to_string();

    loop {
        match parse(&full_input) {
            Ok(parsed_input) => {
                return Ok(parsed_input);
            }

            Err(_) => {
                print!("> ");
                stdout().flush().unwrap();

                let mut next_line = String::new();
                match stdin().read_line(&mut next_line) {
                    Ok(0) => {
                        return Err(ShellError::Other("Unclosed quote".to_string()));
                    }
                    Ok(_) => {
                        full_input.push('\n');
                        full_input.push_str(&next_line.trim_end());
                    }
                    Err(e) => return Err(ShellError::IoError(e)),
                }
            }
        }
    }
}

fn parse(input: &str) -> Result<Vec<String>, ShellError> {
    let mut result = Vec::new();
    let mut current_word = String::new();
    let mut state = ParseState::Normal;
    let mut chars = input.chars();

    while let Some(ch) = chars.next() {
        match state {
            ParseState::Normal => match ch {
                ' ' => {
                    if !current_word.is_empty() {
                        result.push(current_word.clone());
                        current_word.clear();
                    }
                }
                '\'' => {
                    state = ParseState::SingleQuote;
                }
                '"' => {
                    state = ParseState::DoubleQuote;
                }
                '\\' => {
                    if let Some(next_ch) = chars.next() {
                        current_word.push(next_ch);
                    }
                }
                _ => {
                    current_word.push(ch);
                }
            },

            ParseState::SingleQuote => match ch {
                '\'' => state = ParseState::Normal,
                _ => current_word.push(ch),
            },

            ParseState::DoubleQuote => match ch {
                '"' => state = ParseState::Normal,
                '\\' => {
                    if let Some(next_ch) = chars.next() {
                        if next_ch == '\\' {
                            current_word.push(ch);
                        } else {
                            current_word.push('\\');
                            current_word.push(next_ch);
                        }
                    } else {
                        current_word.push('\\');
                    }
                }
                _ => current_word.push(ch),
            },
        }
    }

    match state {
        ParseState::DoubleQuote | ParseState::SingleQuote => {
            return Err(ShellError::Other("unclosed quote".to_string()));
        }
        ParseState::Normal => {
            if !current_word.is_empty() {
                result.push(current_word);
            }
        }
    }

    Ok(result)
}
