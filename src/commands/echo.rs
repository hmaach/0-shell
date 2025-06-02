use crate::commands::Command;
use crate::error::*;

pub struct EchoCommand;

impl Command for EchoCommand {
    fn execute(&self, args: Vec<String>) -> Result<(), ShellError> {
        if args.is_empty() {
            println!();
            return Ok(());
        }

        let parsed_args = args.iter().map(|arg| process_escape(arg)).collect::<Vec<String>>().join(" ");

        println!("{}", parsed_args);
        Ok(())
    }
}

fn process_escape(arg: &str) -> String {
    let mut chars = arg.chars();
    let mut result = String::new();

    while let Some(ch) = chars.next() {
        match ch {
            '\\' => {
                if let Some(next_char) = chars.next() {
                    match next_char {
                        'n' => result.push('\n'),
                        'r' => result.push('\r'),
                        't' => result.push('\t'),
                        '\\' => result.push('\\'),
                        _ => {
                            result.push('\\');
                            result.push(next_char);
                        }
                    }
                }
            }
            _ => {
                result.push(ch);
            }
        }
    }

    result
}