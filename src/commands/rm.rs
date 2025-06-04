use std::{env, fs};

use crate::commands::Command;
use crate::error::*;

pub struct RmCommand;

impl Command for RmCommand {
    fn execute(&self, args: Vec<String>) -> Result<(), ShellError> {
        if args.is_empty() {
            return Err(ShellError::Other("missing operand".into()));
        }

        let mut recursive = false;
        let mut targets = Vec::new();

        for arg in args {
            if arg == "-r" {
                recursive = true;
            } else {
                targets.push(arg);
            }
        }

        if targets.is_empty() {
            return Err(ShellError::Other("missing operand".into()));
        }

        let cur_dir = env::current_dir().unwrap();

        for elem in targets {
            if elem.eq(".") || elem.eq("..") {
                return Err(ShellError::Other(format!(
                    "rm: refusing to remove '.' or '..' directory: skipping '{}'",
                    elem
                )));
            }

            let path = cur_dir.join(&elem);
            if !path.exists() {
                return Err(ShellError::Other(format!(
                    "rm: cannot remove '{}': No such file or directory",
                    elem
                )));
            }

            if path.is_file() {
                fs::remove_file(&path).map_err(|e| {
                    ShellError::Other(format!("failed to remove file '{}': {}", elem, e))
                })?;
            } else if path.is_dir() {
                if recursive {
                    fs::remove_dir_all(&path).map_err(|e| {
                        ShellError::Other(format!(
                            "failed to remove directory recursively '{}': {}",
                            elem, e
                        ))
                    })?;
                } else {
                    return Err(ShellError::Other(format!(
                        "cannot remove '{}': Is a directory. Use -r to remove recursively.",
                        elem
                    )));
                }
            } else {
                return Err(ShellError::Other(format!(
                    "rm: cannot remove '{}': Not a regular file or directory",
                    elem
                )));
            }
        }

        Ok(())
    }
}
