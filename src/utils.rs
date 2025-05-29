use std::os::unix::fs::PermissionsExt;
use std::{fs, path::PathBuf};

pub fn parse_command(input: String) -> (String, Vec<String>) {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    if parts.is_empty() {
        return (String::new(), Vec::new());
    }

    let cmd = parts[0].to_string();
    let args = parts[1..].iter().map(|arg| arg.to_string()).collect();

    (cmd, args)
}

pub fn get_permission_string(path: &PathBuf) -> String {
    let metadata = fs::metadata(path).expect("Failed to get metadata");
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
