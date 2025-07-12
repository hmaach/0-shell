use std::collections::HashMap;

use terminal_size::{Width, terminal_size};

use super::{Directory, formatter::format_detailed_file_info, parser::Flag};

pub struct LsOutput;

impl LsOutput {
    pub fn print_results(
        file_result: &Vec<Vec<String>>,
        dir_results: &Vec<Directory>,
        directories_length: &usize,
        files_length: &usize,
        max_files_len: &usize,
        flags: &Flag,
    ) {
        // Print files
        if !file_result.is_empty() {
            let mut file_result_clone = file_result.clone();
            Self::print(&mut file_result_clone, max_files_len, flags);
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
            Self::print(&mut entries_clone, &dir.max_len, flags);
            if i < directories_length - 1 {
                println!();
            }
        }
    }

    fn format_result(result: &Vec<Vec<String>>, _term_width: &usize) -> String {
        let mut res = String::new();
        for (i, path) in result.iter().enumerate() {
            res.push_str(&path[0]);
            if i < result.len() - 1 {
                res.push_str("  ");
            }
        }
        res
    }

    fn print(result: &mut Vec<Vec<String>>, max_size_len: &usize, flags: &Flag) {
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
            for path in result.iter() {
                println!(
                    "{}",
                    format_detailed_file_info(&max_lens, path, &max_size_len)
                );
            }
        } else {
            let term_width = if let Some((Width(w), _)) = terminal_size() {
                w as usize
            } else {
                100
            };

            let res = Self::format_result(result, &term_width);

            print!("{res}");
        }
    }
}
