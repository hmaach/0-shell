use std::path::PathBuf;

use crate::{
    commands::{
        Command,
        ls::{output::LsOutput, processor::LsProcessor},
    },
    error::ShellError,
};

pub use crate::commands::ls::parser::Flag;

mod file_info;
mod file_permissions;
mod formatter;
mod output;
mod parser;
mod processor;

#[derive(Clone, Debug)]
pub struct Directory {
    pub path: PathBuf,
    pub entries: Vec<Vec<String>>,
    pub max_len: usize,
    pub total_blocks: u64,
}

pub struct LsCommand;

impl Command for LsCommand {
    fn execute(&self, args: Vec<String>) -> Result<(), ShellError> {
        let mut directories: Vec<PathBuf> = Vec::new();
        let mut files: Vec<PathBuf> = Vec::new();
        let mut file_result: Vec<Vec<String>> = Vec::new();
        let mut dir_results: Vec<Directory> = Vec::new();

        let flags = Flag::parse(&args, &mut directories, &mut files)?;

        if directories.is_empty() && files.is_empty() {
            directories.push(PathBuf::from("."));
        }
        let mut max_files_len = 0;
        LsProcessor::process_files(&files, &flags, &mut max_files_len, &mut file_result)?;
        LsProcessor::process_directories(&directories, &flags, &mut dir_results)?;

        LsOutput::print_results(
            &file_result,
            &dir_results,
            &directories.len(),
            &files.len(),
            &max_files_len,
            &flags,
        );

        Ok(())
    }
}
