use std::fs;

use crate::commands::Command;
use crate::error::*;

pub struct LsCommand;

impl Command for LsCommand {
    fn execute(&self, _args: Vec<String>) -> Result<(), ShellError> {
        let mut dir = "./";

        let mut cleaned_paths: Vec<String> = fs::read_dir(dir)
            .unwrap()
            .filter_map(Result::ok)
            .filter_map(|entry| {
                entry.path().to_str().and_then(|s| {
                    if s.starts_with("./.") { // skip hiden files
                        None
                    } else {
                        Some(s.strip_prefix("./").unwrap_or(s).to_string())
                    }
                })
            })
            .collect();

        cleaned_paths.sort();

        // Print each cleaned path
        for path in cleaned_paths {
            print!("{} ", path);
        }
        println!();
        Ok(())
    }
}
