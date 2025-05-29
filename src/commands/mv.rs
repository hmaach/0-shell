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
            move_multiple_files(sources, dest_path)
        } else {
            move_single_source(source_path, dest_path)
        }
    }
}

fn move_single_source(source: &Path, dest: &Path) -> Result<(), ShellError> {
    if dest.is_dir() {
        let dest = dest.join(source.file_name().unwrap());
        fs::rename(source, &dest)?;
    } else {
        fs::rename(source, dest)?;
    }

    Ok(())
}

fn move_multiple_files(_sources: &[String], _dest: &Path) -> Result<(), ShellError> {
    Ok(())
}
