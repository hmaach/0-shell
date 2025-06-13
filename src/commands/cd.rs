use std::{env, path::Path};

use crate::commands::Command;
use crate::error::*;

pub struct CdCommand;

impl Command for CdCommand {
    fn execute(&self, args: Vec<String>) -> Result<(), ShellError> {
        match args.len() {
            0 => change_to_home(),
            1 => match args[0].as_str() {
                "-" => change_to_previous(),
                "~" => change_to_home(),
                path if path.starts_with("~/") => match env::var("HOME") {
                    Ok(home_dir) => {
                        let expanded_path = path.replace("~", &home_dir);
                        change_dir(&expanded_path)
                    }
                    Err(_) => Err(ShellError::Other(
                        "cd: HOME environment variable not set".to_string(),
                    )),
                },
                path => change_dir(path),
            },
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

fn change_to_previous() -> Result<(), ShellError> {
    match env::var("OLDPWD") {
        Ok(old_dir) => {
            let current_dir = env::current_dir().map_err(|e| {
                ShellError::Other(format!("cd: cannot get current directory: {}", e))
            })?;

            change_dir(&old_dir)?;

            unsafe { env::set_var("OLDPWD", current_dir) };

            println!("{}", old_dir);

            Ok(())
        }
        Err(_) => Err(ShellError::Other("cd: OLDPWD not set".to_string())),
    }
}

fn change_dir(path: &str) -> Result<(), ShellError> {
    if let Ok(current_dir) = env::current_dir() {
        unsafe { env::set_var("OLDPWD", current_dir) };
    }

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
