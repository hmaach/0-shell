use std::{
    fs::Metadata,
    os::unix::fs::{FileTypeExt, PermissionsExt},
};

pub fn format_permissions(metadata: &Metadata) -> String {
    let mode = metadata.permissions().mode();

    let file_type = metadata.file_type();
    let file_type_char = if file_type.is_dir() {
        'd'
    } else if file_type.is_symlink() {
        'l'
    } else if file_type.is_char_device() {
        'c'
    } else if file_type.is_block_device() {
        'b'
    } else if file_type.is_fifo() {
        'p'
    } else if file_type.is_socket() {
        's'
    } else {
        '-'
    };

    let mut result = String::new();
    result.push(file_type_char);

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
