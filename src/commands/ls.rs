use std::collections::HashMap;
use std::ffi::CStr;
use std::fs::{Metadata, read_dir};
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

use crate::commands::Command;
use crate::error::*;
use crate::utils::{clean_string, colorize};

pub struct LsCommand;

#[derive(Clone)]
struct Directory {
    path: PathBuf,
    entries: Vec<Vec<String>>,
    total_blocks: u64,
}

impl Command for LsCommand {
    fn execute(&self, args: Vec<String>) -> Result<(), ShellError> {
        let mut directories: Vec<PathBuf> = Vec::new();
        let mut files: Vec<PathBuf> = Vec::new();
        let mut file_result: Vec<Vec<String>> = Vec::new();
        let mut dir_results: Vec<Directory> = Vec::new();

        let (a_flag, f_flag, l_flag) = parse_flags(&args, &mut directories, &mut files)?;

        if directories.is_empty() && files.is_empty() {
            directories.push(PathBuf::from("."));
        }

        // Handle files
        for file in &files {
            if l_flag {
                let info = get_detailed_file_info(&file, None)?;
                file_result.push(info);
            } else {
                let name = file
                    .file_name()
                    .and_then(|s| s.to_str())
                    .ok_or_else(|| {
                        ShellError::Other(format!(
                            "ls: Invalid UTF-8 file name: {}",
                            file.display()
                        ))
                    })?
                    .to_string();

                file_result.push(vec![name]);
            }
        }

        // Handle directories
        for dir in directories.iter() {
            let entries = read_dir(&dir).map_err(|e| {
                ShellError::Other(format!(
                    "ls: cannot open directory '{}': {}",
                    dir.display(),
                    e
                ))
            })?;

            let mut dir_entry_result: Vec<Vec<String>> = Vec::new();
            let mut total_blocks: u64 = 0;

            if a_flag {
                add_dot_entries(&mut dir_entry_result, &mut total_blocks, &f_flag, &l_flag)
                    .map_err(|e| {
                        ShellError::Other(format!("ls: Failed to add dot entries: {}", e))
                    })?;
            }

            let mut paths: Vec<_> = entries
                .filter_map(|entry| entry.ok())
                .filter(|entry| {
                    if !a_flag {
                        if let Some(name) = entry.file_name().to_str() {
                            return !name.starts_with('.');
                        }
                    }
                    true
                })
                .collect();

            paths.sort_by(|a, b| {
                let a_name = clean_string(a.file_name().to_string_lossy().to_uppercase());
                let b_name = clean_string(b.file_name().to_string_lossy().to_uppercase());
                a_name.cmp(&b_name)
            });

            for entry in paths {
                let path = entry.path();
                if l_flag {
                    let info = get_detailed_file_info(&path, Some(&mut total_blocks))?;
                    dir_entry_result.push(info);
                } else {
                    let mut name = path
                        .file_name()
                        .and_then(|s| s.to_str())
                        .ok_or_else(|| {
                            ShellError::Other(format!(
                                "ls: Invalid UTF-8 file name: {}",
                                path.display()
                            ))
                        })?
                        .to_string();

                    if path.is_dir() {
                        name = colorize(&name, "blue", true);
                        if f_flag {
                            name.push('/');
                        }
                    }

                    dir_entry_result.push(vec![name]);
                }
            }

            dir_results.push(Directory {
                path: dir.clone(),
                entries: dir_entry_result,
                total_blocks,
            });
        }

        // Print files
        if !file_result.is_empty() {
            print(&mut file_result, &l_flag);
            if !dir_results.is_empty() {
                println!();
            }
        }

        for (i, mut dir) in dir_results.into_iter().enumerate() {
            if directories.len() + files.len() > 1 {
                println!("{}:", dir.path.display());
            }
            println!("total {}:", dir.total_blocks);
            print(&mut dir.entries, &l_flag);
            if i < directories.len() - 1 {
                println!();
            }
        }

        Ok(())
    }
}

fn parse_flags(
    args: &Vec<String>,
    directories: &mut Vec<PathBuf>,
    files: &mut Vec<PathBuf>,
) -> Result<(bool, bool, bool), ShellError> {
    let mut a_flag = false;
    let mut f_flag = false;
    let mut l_flag = false;

    for arg in args {
        if arg.starts_with('-') {
            for ch in arg.chars().skip(1) {
                match ch {
                    'a' => a_flag = true,
                    'F' => f_flag = true,
                    'l' => l_flag = true,
                    _ => {
                        return Err(ShellError::Other(format!(
                            "invalid flag: '{}', supported flags are: '-a', '-F', '-l'",
                            ch
                        )));
                    }
                }
            }
        } else {
            let path = PathBuf::from(arg);
            if path.is_dir() {
                directories.push(path);
            } else if path.is_file() {
                files.push(path);
            } else {
                return Err(ShellError::Other(format!(
                    "cannot access {:?}: No such file or directory",
                    arg
                )));
            }
        }
    }

    Ok((a_flag, f_flag, l_flag))
}

fn get_detailed_file_info(
    path: &PathBuf,
    total_blocks: Option<&mut u64>,
) -> Result<Vec<String>, ShellError> {
    let metadata = path.metadata()?;

    let permission = get_permission_string(&metadata);

    let len = metadata.len().to_string();

    let mut file_name = path
        .file_name()
        .and_then(|s| s.to_str())
        .map(|s| s.to_string())
        .or_else(|| Some(path.to_string_lossy().to_string())) // fallback for "." or ".."
        .ok_or_else(|| {
            ShellError::Other(format!("Unable to get file name for path: {:?}", path))
        })?;

    if path.is_dir() {
        let colored_name = colorize(&file_name, "blue", true);
        file_name = format!("{}/", colored_name);
    }

    let (owner_name, group_name) = get_file_owner_and_group(&metadata);

    let n_link = metadata.nlink().to_string();

    let modified_at = get_modified_at(&metadata);

    if let Some(blocks) = total_blocks {
        *blocks += metadata.blocks() / 2;
    }

    Ok(vec![
        permission,
        n_link,
        owner_name,
        group_name,
        len,
        modified_at,
        file_name,
    ])
}

fn format_detailed_file_info(max_lens: &HashMap<usize, usize>, path: &Vec<String>) -> String {
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

fn get_file_owner_and_group(metadata: &Metadata) -> (String, String) {
    let uid = metadata.uid();
    let gid = metadata.gid();

    let username = unsafe {
        let passwd = libc::getpwuid(uid);
        if passwd.is_null() {
            format!("UID({})", uid)
        } else {
            CStr::from_ptr((*passwd).pw_name)
                .to_string_lossy()
                .into_owned()
        }
    };

    let groupname = unsafe {
        let group = libc::getgrgid(gid);
        if group.is_null() {
            format!("GID({})", gid)
        } else {
            CStr::from_ptr((*group).gr_name)
                .to_string_lossy()
                .into_owned()
        }
    };

    (username, groupname)
}

fn get_modified_at(metadata: &Metadata) -> String {
    match metadata.modified() {
        Ok(modified_at) => {
            let datetime: chrono::DateTime<chrono::Local> = modified_at.into();
            datetime.format("%b %e %H:%M").to_string()
        }
        Err(_) => "<invalid time>".to_string(),
    }
}

pub fn get_permission_string(metadata: &Metadata) -> String {
    let mode = metadata.permissions().mode();

    let file_type = if metadata.is_dir() {
        'd'
    } else if metadata.file_type().is_symlink() {
        'l'
    } else {
        '-'
    };

    let mut result = String::new();
    result.push(file_type);

    let bits = [
        (mode >> 6) & 0b111, // user
        (mode >> 3) & 0b111, // group
        (mode >> 0) & 0b111, // others
    ];

    for &part in &bits {
        result.push(if part & 0b100 != 0 { 'r' } else { '-' });
        result.push(if part & 0b010 != 0 { 'w' } else { '-' });
        result.push(if part & 0b001 != 0 { 'x' } else { '-' });
    }

    result
}

fn add_dot_entries(
    result: &mut Vec<Vec<String>>,
    total_blocks: &mut u64,
    f_flag: &bool,
    l_flag: &bool,
) -> Result<(), ShellError> {
    let mut dot = format!("{}", colorize(".", "blue", true));
    let mut dotdot = format!("{}", colorize("..", "blue", true));

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

fn print(result: &mut Vec<Vec<String>>, is_long: &bool) {
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
