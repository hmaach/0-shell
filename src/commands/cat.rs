use crate::commands::Command;
use crate::error::*;

pub struct CatCommand;

impl Command for CatCommand {
    fn execute(&self, _args: Vec<String>) -> Result<(), ShellError> {
        Ok(())
    }
}