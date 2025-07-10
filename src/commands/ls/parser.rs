use std::path::PathBuf;

use crate::error::ShellError;

pub struct Flag {
    pub l: bool,
    pub a: bool,
    pub f: bool,
}

impl Flag {
    pub fn parse(
        args: &Vec<String>,
        directories: &mut Vec<PathBuf>,
        files: &mut Vec<PathBuf>,
    ) -> Result<Self, ShellError> {
        let mut flags = Self {
            a: false,
            f: false,
            l: false,
        };

        for arg in args {
            if arg.starts_with('-') {
                for ch in arg.chars().skip(1) {
                    match ch {
                        'a' => flags.a = true,
                        'F' => flags.f = true,
                        'l' => flags.l = true,
                        _ => {
                            return Err(ShellError::Other(format!(
                                "invalid flag: '{}', supported flags are: '-a', '-F', '-l'",
                                ch
                            )));
                        }
                    }
                }
            } else {
                let path = PathBuf::from(arg);
                if path.is_dir() {
                    directories.push(path);
                } else if path.is_file() {
                    files.push(path);
                } else {
                    return Err(ShellError::Other(format!(
                        "cannot access {:?}: No such file or directory",
                        arg
                    )));
                }
            }
        }

        Ok(flags)
    }
}
