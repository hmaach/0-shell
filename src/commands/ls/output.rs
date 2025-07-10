use std::collections::HashMap;

use super::{Directory, formatter::format_detailed_file_info, parser::Flag};

pub struct LsOutput;

impl LsOutput {
    pub fn print_results(
        file_result: &Vec<Vec<String>>,
        dir_results: &Vec<Directory>,
        directories_length: &usize,
        files_length: &usize,
        flags: &Flag,
    ) {
        // Print files
        if !file_result.is_empty() {
            let mut file_result_clone = file_result.clone();
            Self::print(&mut file_result_clone, flags);
            if !dir_results.is_empty() {
                println!();
            }
        }

        // Print directories
        for (i, dir) in dir_results.iter().enumerate() {
            if directories_length + files_length > 1 {
                println!("{}:", dir.path.display());
            }

            if flags.l {
                println!("total {}:", dir.total_blocks);
            }

            let mut entries_clone = dir.entries.clone();
            Self::print(&mut entries_clone, flags);
            if i < directories_length - 1 {
                println!();
            }
        }
    }

    pub fn print(result: &mut Vec<Vec<String>>, flags: &Flag) {
        let mut max_lens: HashMap<usize, usize> = HashMap::new();

        if flags.l {
            for path in result.iter() {
                for (i, field) in path.iter().enumerate() {
                    let len = field.len();
                    let entry = max_lens.entry(i).or_insert(0);
                    if len > *entry {
                        *entry = len;
                    }
                }
            }
        }

        for (i, path) in result.iter().enumerate() {
            if flags.l {
                println!("{}", format_detailed_file_info(&max_lens, path));
            } else {
                print!("{}", path[0]);
                if i < result.len() - 1 {
                    print!("  ");
                }
            }
        }

        if !flags.l {
            println!();
        }
    }
}
