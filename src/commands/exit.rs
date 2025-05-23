use crate::commands::Command;
use crate::error::ShellError;

pub struct ExitCommand;

impl Command for ExitCommand {
    fn execute(&self, _args: Vec<String>) -> Result<(), ShellError> {
        println!("Exiting shell");
        Ok(())
    }
}
