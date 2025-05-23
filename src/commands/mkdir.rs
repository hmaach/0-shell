use std::fs;
use crate::commands::Command;
use crate::error::*;

pub struct MkdirCommand;

impl Command for MkdirCommand {
    fn execute(&self, args: Vec<String>) -> Result<(), ShellError> {
        if args.is_empty() {
            return Err(ShellError::Other("mkdir: missing operand".to_owned()));
        }
        for dir_name in args {
            if let Err(err) = fs::create_dir(&dir_name) {
                return Err(ShellError::Other(err.to_string()));
            }
        }

        Ok(())
    }
}
