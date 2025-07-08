use std::{collections::HashMap, path::PathBuf};

use crate::{
    commands::ls::file_info::get_detailed_file_info,
    error::ShellError,
    utils::{Color, colorize},
};

pub fn add_dot_entries(
    result: &mut Vec<Vec<String>>,
    total_blocks: &mut u64,
    f_flag: &bool,
    l_flag: &bool,
) -> Result<(), ShellError> {
    let mut dot = format!("{}", colorize(".", Color::Blue, true));
    let mut dotdot = format!("{}", colorize("..", Color::Blue, true));

    if *f_flag {
        dot.push('/');
        dotdot.push('/');
    };

    if *l_flag {
        let dot_path = PathBuf::from(".");
        let dotdot_path = PathBuf::from("..");

        let mut dot_info = get_detailed_file_info(&dot_path, Some(total_blocks))?;
        let mut dotdot_info = get_detailed_file_info(&dotdot_path, Some(total_blocks))?;

        dot_info[6] = dot;
        dotdot_info[6] = dotdot;

        result.insert(0, dotdot_info);
        result.insert(0, dot_info);
    } else {
        result.insert(0, vec![dotdot]);
        result.insert(0, vec![dot]);
    }
    Ok(())
}


pub fn format_detailed_file_info(max_lens: &HashMap<usize, usize>, path: &Vec<String>) -> String {
    let mut result = String::new();

    for (i, info) in path.iter().enumerate() {
        let max_width = max_lens.get(&i).copied().unwrap_or(0);

        if i == path.len() - 1 {
            result.push_str(info);
        } else if i == 1 || i == 4 {
            result.push_str(&format!("{:>width$} ", info, width = max_width));
        } else {
            result.push_str(&format!("{:<width$} ", info, width = max_width));
        }
    }

    result
}

pub fn print(result: &mut Vec<Vec<String>>, is_long: &bool) {
    let mut max_lens: HashMap<usize, usize> = HashMap::new();

    if *is_long {
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
        if *is_long {
            println!("{}", format_detailed_file_info(&max_lens, path));
        } else {
            print!("{}", path[0]);
            if i < result.len() - 1 {
                print!("  ");
            }
        }
    }

    if !*is_long {
        println!();
    }
}
