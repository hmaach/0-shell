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
            if arg.starts_with("-") {
                if arg == "-r" {
                    recursive = true;
                } else {
                    return Err(ShellError::Other(format!(
                        "rm: invalid option -- '{}'",
                        arg
                    )));
                }
            } else {
                targets.push(arg);
            }
        }

        if targets.is_empty() {
            return Err(ShellError::Other("missing operand".into()));
        }

        let cur_dir = env::current_dir().map_err(|e| {
            ShellError::Other(format!("rm: failed to get current directory: {}", e))
        })?;

        for elem in targets {
            if elem == "." || elem == ".." || elem == "./." || elem == "./.." {
                eprintln!(
                    "rm: refusing to remove '.' or '..' directory: skipping '{}'",
                    elem
                );
                continue;
            }

            let path = cur_dir.join(&elem);
            if !path.exists() {
                eprintln!("rm: cannot remove '{}': No such file or directory", elem);
                continue;
            }

            if path.is_file() {
                if let Err(e) = fs::remove_file(&path) {
                    eprintln!("rm: failed to remove file '{}': {}", elem, e);
                }
            } else if path.is_dir() {
                if recursive {
                    if let Err(e) = fs::remove_dir_all(&path) {
                        eprintln!(
                            "rm: failed to remove directory recursively '{}': {}",
                            elem, e
                        );
                    }
                } else {
                    eprintln!(
                        "rm: cannot remove '{}': Is a directory. Use -r to remove recursively.",
                        elem
                    );
                }
            } else {
                eprintln!(
                    "rm: cannot remove '{}': Not a regular file or directory",
                    elem
                );
            }
        }

        Ok(())
    }
}
