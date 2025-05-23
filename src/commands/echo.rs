use crate::commands::Command;
use crate::error::*;

pub struct EchoCommand;

impl Command for EchoCommand {
    fn execute(&self, args: Vec<String>) -> Result<(), ShellError> {
        let text = args.join(" ");
        println!("{}", text);
        Ok(())
    }
}