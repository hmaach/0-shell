use std::{
    collections::HashMap,
    fs,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
};

use super::{file_info::get_detailed_file_info, parser::Flag};

use crate::{
    color::{Color, colorize, colorize_dir, colorize_executable, colorize_symlink},
    error::ShellError,
};

pub fn add_dot_entries(
    result: &mut Vec<Vec<String>>,
    total_blocks: &mut u64,
    flags: &Flag,
) -> Result<(), ShellError> {
    let mut dot = ".".to_owned();
    let mut dotdot = "..".to_owned();

    colorize_dir(&mut dot, flags);
    colorize_dir(&mut dotdot, flags);

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

pub fn format_path(path: &PathBuf, file_name: &mut String, flags: &Flag) -> Result<(), ShellError> {
    let metadata = path.symlink_metadata()?;
    let mode = metadata.permissions().mode();

    if path.is_symlink() {
        return format_symlink(path, file_name, flags);
    } else if path.is_dir() {
        colorize_dir(file_name, flags);
        return Ok(());
    }

    if is_executable(mode) {
        colorize_executable(file_name, flags);
    }

    Ok(())
}

fn is_executable(mode: u32) -> bool {
    mode & 0o111 != 0
}

fn format_symlink(path: &PathBuf, file_name: &mut String, flags: &Flag) -> Result<(), ShellError> {
    let is_broken = fs::metadata(path).is_err();
    colorize_symlink(file_name, is_broken, flags);

    if flags.l {
        if let Ok(target) = fs::read_link(path) {
            let full_target_path = if target.is_absolute() {
                target.clone()
            } else {
                path.parent().unwrap_or_else(|| Path::new("")).join(&target)
            };

            let mut target_str = target.to_string_lossy().to_string();

            if fs::metadata(&full_target_path).is_err() {
                colorize_symlink(&mut target_str, true, flags);
            } else if !target.is_symlink() {
                let _ = format_path(&full_target_path, &mut target_str, flags);
            } // here I need to put formatting for the the symlink

            file_name.push_str(" -> ");
            file_name.push_str(&target_str);
        } else {
            file_name.push_str(" -> ");
            file_name.push_str(&colorize("invalid symlink", Color::Red, true));
        }
    }

    Ok(())
}
