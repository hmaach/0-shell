use std::env;

use crate::commands::Command;
use crate::error::*;

pub struct PwdCommand;

impl Command for PwdCommand {
    fn execute(&self, _args: Vec<String>) -> Result<(), ShellError> {
        match env::current_dir() {
            Ok(path) => {
                println!("{}", path.display());
                Ok(())
            },
            Err(err) => Err(ShellError::Other(format!("error happened: {}", err)))
        }
    }
}