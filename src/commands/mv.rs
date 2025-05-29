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

        let source_path = Path::new(&sources[0]);
        let dest_path = Path::new(&dest[0]);

        fs::rename(source_path, dest_path).expect("error in rename func");
        Ok(())
    }
}
