use crate::commands::Command;
use crate::error::*;

pub struct CpCommand;

impl Command for CpCommand {
    fn execute(&self, _args: Vec<String>) -> Result<(), ShellError> {
        Ok(())
    }
}