use std::path::PathBuf;

use crate::error::ShellError;

pub fn parse_flags(
    args: &Vec<String>,
    directories: &mut Vec<PathBuf>,
    files: &mut Vec<PathBuf>,
) -> Result<(bool, bool, bool), ShellError> {
    let mut a_flag = false;
    let mut f_flag = false;
    let mut l_flag = false;

    for arg in args {
        if arg.starts_with('-') {
            for ch in arg.chars().skip(1) {
                match ch {
                    'a' => a_flag = true,
                    'F' => f_flag = true,
                    'l' => l_flag = true,
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

    Ok((a_flag, f_flag, l_flag))
}
