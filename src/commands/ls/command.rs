use std::path::PathBuf;

use crate::commands::Command;
use crate::commands::ls::output::LsOutput;
use crate::commands::ls::parser::Flag;
use crate::commands::ls::processor::LsProcessor;
use crate::error::ShellError;

#[derive(Clone, Debug)]
pub struct Directory {
    pub path: PathBuf,
    pub entries: Vec<Vec<String>>,
    pub total_blocks: u64,
}

pub struct LsCommand;

impl Command for LsCommand {
    fn execute(&self, args: Vec<String>) -> Result<(), ShellError> {
        let mut directories: Vec<PathBuf> = Vec::new();
        let mut files: Vec<PathBuf> = Vec::new();
        let mut file_result: Vec<Vec<String>> = Vec::new();
        let mut dir_results: Vec<Directory> = Vec::new();

        // Parse command line arguments
        let flags = Flag::parse(&args, &mut directories, &mut files)?;

        // Default to current directory if no paths specified
        if directories.is_empty() && files.is_empty() {
            directories.push(PathBuf::from("."));
        }

        // Process files and directories
        LsProcessor::process_files(&files, &flags, &mut file_result)?;
        LsProcessor::process_directories(&directories, &flags, &mut dir_results)?;

        // Print results
        LsOutput::print_results(
            &file_result,
            &dir_results,
            &directories.len(),
            &files.len(),
            &flags,
        );

        Ok(())
    }
}
