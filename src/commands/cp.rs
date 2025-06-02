use std::env;
use std::fs::copy;
use std::path::{Path, PathBuf};

use crate::commands::Command;
use crate::error::*;

pub struct CpCommand;

impl Command for CpCommand {
    fn execute(&self, args: Vec<String>) -> Result<(), ShellError> {
        let src = args[0..args.len() - 1].to_vec();
        let target = args
            .iter()
            .last()
            .expect("args should not be empty")
            .clone();
        let is_target_dir = Path::new::<String>(&target).is_dir();
        let one_file = src.len() == 1 && Path::new::<String>(&src[0]).is_file();
        let is_source_exist = Path::new::<String>(&src[0]).exists();

        if !is_source_exist && !is_target_dir {
            return Err(ShellError::Other(format!(
                "cp: cannot stat '{}': No such file",
                &src[0]
            )));
        }

        if !is_target_dir && one_file {
            let src_path = if Path::new(&src[0]).is_relative() {
                PathBuf::from(env::current_dir().unwrap()).join(&src[0])
            } else {
                PathBuf::from(&src[0])
            };

            let target_path = if Path::new(&target).is_relative() {
                PathBuf::from(env::current_dir().unwrap()).join(&target)
            } else {
                PathBuf::from(&target)
            };

            if src_path == target_path {
                return Err(ShellError::Other(format!(
                    "cp: '{}' and '{}' are the same file",
                    &src[0], &target
                )));
            }

            if let Err(e) = copy(&src[0], &target) {
                return Err(ShellError::Other(format!(
                    "cp: cannot copy '{}' to '{}': {}",
                    &src[0], &target, e
                )));
            }
        }

        if !is_target_dir && src.len() > 1 {
            return Err(ShellError::Other(format!(
                "cp: target '{}' is not a directory",
                target
            )));
        }

        let mut errors = Vec::new();

        if is_target_dir {
            for s in src {
                let path = Path::new(&s);
                if !path.exists() {
                    errors.push(format!(
                        "cp: cannot stat '{}': No such file or directory",
                        s
                    ));
                    continue;
                }

                if path.is_dir() {
                    errors.push(format!("cp: -r not specified; omitting directory '{}'", s));
                    continue;
                }

                let mut dist = target.clone();
                dist.push('/');

                match path.file_name() {
                    Some(filename) => {
                        dist.push_str(&filename.to_string_lossy());
                    }
                    None => {
                        errors.push(format!("cp: cannot determine filename for '{}'", s));
                        continue;
                    }
                }

                if let Err(e) = copy(path, &dist) {
                    errors.push(format!("cp: cannot copy '{}' to '{}': {}", s, dist, e));
                }
            }
        }

        if !errors.is_empty() {
            return Err(ShellError::Other(errors.join("\n")));
        }

        Ok(())
    }
}
