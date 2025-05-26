use crate::commands::Command;
use crate::error::*;

pub struct EchoCommand;

impl Command for EchoCommand {
    fn execute(&self, args: Vec<String>) -> Result<(), ShellError> {
        if args.is_empty() {
            println!();
            return Ok(());
        }
        let processed_args: Result<Vec<String>, ShellError> =
            args.iter().map(|arg| process_arg(arg)).collect();

        match processed_args {
            Ok(args) => {
                let text = args.join(" ");
                println!("{}", text);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}

fn process_arg(arg: &str) -> Result<String, ShellError> {
    if arg.contains('`') {
        return Err(ShellError::Backticks);
    }

    if arg.starts_with('"') {
        process_double_quote(arg)
    } else if arg.starts_with('\'') {
        process_single_quote(arg)
    } else {
        Ok(arg.to_string())
    }
}

fn process_double_quote(arg: &str) -> Result<String, ShellError> {
    if arg.len() < 2 {
        return Err(ShellError::Other("Unclosed single quote".to_string()));
    }

    if !arg.ends_with('"') {
        return Err(ShellError::Other("unclosed double quote".to_string()));
    }

    let chars: Vec<char> = arg.chars().collect();
    let mut result = String::new();
    let mut i = 1;

    while i < chars.len() - 1 {
        if chars[i] == '\\' && i + 1 < chars.len() - 1 {
            match chars[i + 1] {
                '\\' => {
                    result.push(chars[i]);
                    i += 2;
                }
                _ => i += 1,
            }
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }

    Ok(result)
}

fn process_single_quote(arg: &str) -> Result<String, ShellError> {
    if arg.len() < 2 {
        return Err(ShellError::Other("Unclosed single quote".to_string()));
    }

    if !arg.ends_with('\'') {
        return Err(ShellError::Other("Unclosed single quote".to_string()));
    }

    let content: String = arg[1..arg.len() - 1].to_string();

    Ok(content)
}
