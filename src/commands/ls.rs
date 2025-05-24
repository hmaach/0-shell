use crate::commands::Command;
use crate::error::*;

pub struct LsCommand;

impl Command for LsCommand {
    fn execute(&self, _args: Vec<String>) -> Result<(), ShellError> {
        
        Ok(())
    }
}
