use std::os::unix::fs::PermissionsExt;
use std::{collections::HashMap, fs, path::PathBuf};

use crate::commands::ls::parser::Flag;
use crate::{
    commands::ls::file_info::get_detailed_file_info,
    error::ShellError,
    utils::{Color, colorize},
};

pub fn add_dot_entries(
    result: &mut Vec<Vec<String>>,
    total_blocks: &mut u64,
    flags: &Flag,
) -> Result<(), ShellError> {
    let mut dot = format!("{}", colorize(".", Color::Blue, true));
    let mut dotdot = format!("{}", colorize("..", Color::Blue, true));

    if flags.f {
        dot.push('/');
        dotdot.push('/');
    };

    if flags.l {
        let dot_path = PathBuf::from(".");
        let dotdot_path = PathBuf::from("..");

        let mut dot_info = get_detailed_file_info(&dot_path, Some(total_blocks), flags)?;
        let mut dotdot_info = get_detailed_file_info(&dotdot_path, Some(total_blocks), flags)?;

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

pub fn format_path(path: &PathBuf, file_name: &mut String, flags: &Flag) -> Result<(), ShellError> {
    let metadata = path.symlink_metadata()?;
    let mode = metadata.permissions().mode();

    if path.is_dir() {
        let colored_name = colorize(&file_name, Color::Blue, true);
        *file_name = format!("{}", colored_name);
        if flags.f {
            file_name.push('/');
        }
    } else if path.is_symlink() {
        let colored_name = colorize(&file_name, Color::SkyBlue, true);
        *file_name = format!("{}", colored_name);
        if flags.f && !flags.l {
            file_name.push('@');
        }
        if flags.l {
            let target: PathBuf = fs::read_link(path)?;
            let mut target_str = target.to_string_lossy().to_string();

            format_path(&target, &mut target_str, flags)?; // colorize the *target* name

            file_name.push_str(" -> ");
            file_name.push_str(&target_str);
        }
    } else if mode & 0o111 != 0 {
        let colored_name = colorize(&file_name, Color::Green, true);
        *file_name = format!("{}", colored_name);
        if flags.f {
            file_name.push('*');
        }
    }
    Ok(())
}
