use std::collections::HashMap;

use terminal_size::{Width, terminal_size};

use crate::utils::strip_ansi_codes;

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

    fn format_result(result: &Vec<Vec<String>>, term_width: usize) -> String {
        if result.is_empty() {
            return String::new();
        }

        let items: Vec<(String, usize)> = result
            .iter()
            .map(|s| {
                let name = s.first().cloned().unwrap_or_default();
                let clean_len = strip_ansi_codes(&name).len();
                (name, clean_len)
            })
            .collect();

        let item_count = items.len();
        let mut best_cols = 1;
        let mut best_rows = item_count;

        if item_count == 1 {
            return format!("{}\n", items[0].0);
        }

        for cols in 1..=item_count.min(term_width / 3) {
            let rows = item_count.div_ceil(cols);

            if rows > best_rows {
                continue;
            }

            let mut col_widths = vec![0; cols];

            for (idx, (_, clean_len)) in items.iter().enumerate() {
                let col = idx / rows;
                if col < cols {
                    col_widths[col] = col_widths[col].max(*clean_len);
                }
            }

            let total_width: usize = col_widths.iter().sum::<usize>() + (cols - 1) * 2;

            if total_width <= term_width {
                best_cols = cols;
                best_rows = rows;

                if rows == 1 {
                    break;
                }
            }
        }

        let cols = best_cols;
        let rows = best_rows;

        let mut col_widths = vec![0; cols];
        for (idx, (_, clean_len)) in items.iter().enumerate() {
            let col = idx / rows;
            if col < cols {
                col_widths[col] = col_widths[col].max(*clean_len);
            }
        }

        let estimated_capacity = (term_width + 1) * rows;
        let mut result = String::with_capacity(estimated_capacity);

        for row in 0..rows {
            for (col, col_width) in col_widths.iter().enumerate() {
                let idx = col * rows + row;
                if idx < item_count {
                    let (name, clean_len) = &items[idx];
                    result.push_str(name);

                    if col < cols - 1 && idx < item_count - 1 {
                        let pad = col_width - clean_len + 2;
                        result.extend(std::iter::repeat_n(' ', pad));
                    }
                }
            }
            result.push('\n');
        }

        result
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
                    format_detailed_file_info(&max_lens, path, max_size_len)
                );
            }
        } else {
            let term_width = if let Some((Width(w), _)) = terminal_size() {
                w as usize
            } else {
                100
            };

            let res = Self::format_result(result, term_width);

            print!("{res}");
        }
    }
}
