use crate::commands::Command;
use crate::error::*;
use std::fs;

pub struct MkdirCommand;

impl Command for MkdirCommand {
    fn execute(&self, args: Vec<String>) -> Result<(), ShellError> {
        if args.is_empty() {
            return Err(ShellError::Other("mkdir: missing operand".to_owned()));
        }

        let mut errors = Vec::new();

        for dir_name in args {
            if let Err(err) = fs::create_dir(&dir_name) {
                errors.push(format!("mkdir: {}: {}", dir_name, err));
            }
        }

        if !errors.is_empty() {
            return Err(ShellError::Other(errors.join("\n")));
        }

        Ok(())
    }
}
