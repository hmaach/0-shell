use crate::commands::ls::command::Directory;

use super::formatter::print;

pub struct LsOutput;

impl LsOutput {
    pub fn print_results(
        file_result: &Vec<Vec<String>>,
        dir_results: &Vec<Directory>,
        directories_length: &usize,
        files_length: &usize,
        l_flag: &bool,
    ) {
        // Print files
        if !file_result.is_empty() {
            let mut file_result_clone = file_result.clone();
            print(&mut file_result_clone, l_flag);
            if !dir_results.is_empty() {
                println!();
            }
        }

        // Print directories
        for (i, dir) in dir_results.iter().enumerate() {
            if directories_length + files_length > 1 {
                println!("{}:", dir.path.display());
            }

            if *l_flag {
                println!("total {}:", dir.total_blocks);
            }

            let mut entries_clone = dir.entries.clone();
            print(&mut entries_clone, l_flag);
            if i < directories_length - 1 {
                println!();
            }
        }
    }
}
