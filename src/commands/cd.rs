use std::{env, path::Path};

use crate::commands::Command;
use crate::error::*;

pub struct CdCommand;

impl Command for CdCommand {
    fn execute(&self, args: Vec<String>) -> Result<(), ShellError> {
        if let Err(err) = env::set_current_dir(Path::new(&args[0])) {
            return Err(ShellError::Other(err.to_string()));
        }
        Ok(())
    }
}
