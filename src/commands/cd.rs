use std::{env, path::Path};

use crate::commands::Command;
use crate::error::*;

pub struct CdCommand;

impl Command for CdCommand {
    fn execute(&self, args: Vec<String>) -> Result<(), ShellError> {
        match args.len() {
            0 => change_to_home(),
            1 => change_dir(&args[0]),
            _ => Err(ShellError::Other("cd: too many arguments".to_string())),
        }
    }
}

fn change_to_home() -> Result<(), ShellError> {
    match env::var("HOME") {
        Ok(home_dir) => change_dir(&home_dir),
        Err(_) => Err(ShellError::Other(
            "cd: HOME environment variable not set".to_string(),
        )),
    }
}

fn change_dir(path: &str) -> Result<(), ShellError> {
    let target_path = Path::new(path);

    if !target_path.exists() {
        return Err(ShellError::Other(format!(
            "cd: {}: No such file or directory",
            path
        )));
    }

    if !target_path.is_dir() {
        return Err(ShellError::Other(format!("cd: {}: Not a directory", path)));
    }

    if let Err(err) = env::set_current_dir(target_path) {
        return Err(ShellError::Other(err.to_string()));
    }

    Ok(())
}
