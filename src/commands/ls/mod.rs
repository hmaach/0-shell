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
    pub total_blocks: u64,
}

pub struct FileDetails {
    pub file_name: String,
    pub permission: String,
    pub n_link: u32,
    pub owner_name: String,
    pub group_name: String,
    pub minor: Option<u32>,
    pub major: Option<u32>,
    pub len: usize,
    pub modified_at: String,
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

        LsProcessor::process_files(&files, &flags, &mut file_result)?;
        LsProcessor::process_directories(&directories, &flags, &mut dir_results)?;

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
