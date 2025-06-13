use std::fs;
use std::path::Path;

use crate::commands::Command;
use crate::error::ShellError;

pub struct MvCommand;

impl Command for MvCommand {
    fn execute(&self, args: Vec<String>) -> Result<(), ShellError> {
        if args.is_empty() {
            return Err(ShellError::ArgsNotFound("mv".to_string()));
        }

        if args.len() == 1 {
            return Err(ShellError::Other(format!(
                "mv: missing destination file operand after '{}'",
                args[0]
            )));
        }

        let (sources, dest) = args.split_at(args.len() - 1);
        let dest_path = Path::new(&dest[0]);
        let source_path = Path::new(&sources[0]);

        if sources.len() > 1 {
            move_multiple_sources(sources, dest_path)
        } else {
            move_single_source(source_path, dest_path)
        }
    }
}

fn move_single_source(source: &Path, dest: &Path) -> Result<(), ShellError> {
    if !source.exists() {
        return Err(ShellError::Other(format!(
            "mv: cannot stat '{}': No such file or directory",
            source.display()
        )));
    }

    if dest.is_dir() {
        if source == dest {
            return Err(ShellError::Other(format!(
                "mv: cannot move '{}' to a subdirectory of itself, '{}'",
                source.display(),
                dest.display()
            )));
        }

        match source.file_name() {
            Some(file_name) => {
                let dest = dest.join(file_name);
                fs::rename(source, dest)?;
            }
            _ => {
                fs::rename(source, dest)?;
            }
        }
    } else {
        if source == dest {
            return Err(ShellError::Other(format!(
                "mv: '{}' and '{}' are the same file",
                source.display(),
                dest.display()
            )));
        }
        fs::rename(source, dest)?;
    }

    Ok(())
}

fn move_multiple_sources(sources: &[String], dest: &Path) -> Result<(), ShellError> {
    if !dest.is_dir() {
        return Err(ShellError::Other(format!(
            "mv: target '{}' is not a directory",
            dest.display()
        )));
    }

    let mut errors = Vec::new();

    for source in sources {
        let source_path = Path::new(source);

        if !source_path.exists() {
            errors.push(format!(
                "mv: cannot stat '{}': No such file or directory",
                source_path.display()
            ));
        }

        match source_path.file_name() {
            Some(file_name) => {
                let dest_path = dest.join(file_name);
                fs::rename(source_path, dest_path)?;
            }
            _ => {
                fs::rename(source_path, dest)?;
            }
        }
    }

    if !errors.is_empty() {
        return Err(ShellError::Other(errors.join("\n")));
    }

    Ok(())
}
