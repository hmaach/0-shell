use std::collections::HashMap;
use std::ffi::CStr;
use std::fs::{self, Metadata};
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

use crate::commands::Command;
use crate::error::*;
use crate::utils::{clean_string, colorize};

pub struct LsCommand;

impl Command for LsCommand {
    fn execute(&self, args: Vec<String>) -> Result<(), ShellError> {
        let dir = "./";

        let (a_flag, f_flag, l_flag) = match parse_flags(&args) {
            Ok(flags) => flags,
            Err(err) => return Err(ShellError::Other(format!("ls: {}", err))),
        };

        let mut result: Vec<Vec<String>> = Vec::new();
        let mut total_blocks: u64 = 0;

        if a_flag {
            if let Err(e) = add_dot_entries(&mut result, &mut total_blocks, &f_flag, &l_flag) {
                eprintln!("ls: Failed to add dot entries: {}", e);
                return Err(e);
            }
        }

        let entries = match fs::read_dir(dir) {
            Ok(e) => e,
            Err(e) => {
                return Err(ShellError::Other(format!(
                    "ls: Failed to read directory: {}",
                    e
                )));
            }
        };

        let mut paths: Vec<_> = entries
            .filter_map(|entry| match entry {
                Ok(e) => Some(e),
                Err(e) => {
                    eprintln!("ls: Failed to read entry: {}", e);
                    None
                }
            })
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

        result.extend(paths.into_iter().filter_map(|entry| {
            let path = entry.path();

            if l_flag {
                match get_detailed_file_info(&path, &mut total_blocks) {
                    Ok(info) => Some(info),
                    Err(e) => {
                        eprintln!("ls: Failed to get detailed info for {:?}: {}", path, e);
                        None
                    }
                }
            } else {
                let name_result = path.file_name().and_then(|s| s.to_str());
                let mut name = match name_result {
                    Some(s) => s.to_string(),
                    None => {
                        eprintln!("ls: Invalid UTF-8 file name for path: {:?}", path);
                        return None;
                    }
                };

                if path.is_dir() {
                    name = colorize(&name, "blue", true);
                    if f_flag {
                        name.push('/');
                    }
                }

                Some(vec![name])
            }
        }));

        print(&mut result, total_blocks, &l_flag);

        Ok(())
    }
}

fn parse_flags(args: &Vec<String>) -> Result<(bool, bool, bool), ShellError> {
    let mut a_flag = false;
    let mut f_flag = false;
    let mut l_flag = false;

    for arg in args {
        if !arg.starts_with('-') {
            return Err(ShellError::Other(format!("Invalid argument: {:?}", arg)));
        }

        for ch in arg.chars().skip(1) {
            // Skip the leading '-'
            match ch {
                'a' => a_flag = true,
                'F' => f_flag = true,
                'l' => l_flag = true,
                _ => {
                    return Err(ShellError::Other(format!(
                        "Invalid flag: '{}', supported flags are: '-a', '-F', '-l'",
                        ch
                    )));
                }
            }
        }
    }

    Ok((a_flag, f_flag, l_flag))
}

fn get_detailed_file_info(
    path: &PathBuf,
    total_blocks: &mut u64,
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

    *total_blocks += metadata.blocks() / 2;

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

        let mut dot_info = get_detailed_file_info(&dot_path, total_blocks)?;
        let mut dotdot_info = get_detailed_file_info(&dotdot_path, total_blocks)?;

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

fn print(result: &mut Vec<Vec<String>>, total_blocks: u64, is_long: &bool) {
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
        println!("total {total_blocks}");
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
