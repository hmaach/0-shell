use std::{
    fs::Metadata,
    os::unix::fs::{FileTypeExt, PermissionsExt},
};

pub fn format_permissions(metadata: &Metadata) -> String {
    let mode = metadata.permissions().mode();
    let mut permissions = String::new();

    // file type
    permissions.push(get_file_type(metadata));

    // owner permissions
    permissions.push(if mode & 0o400 != 0 { 'r' } else { '-' });
    permissions.push(if mode & 0o200 != 0 { 'w' } else { '-' });
    permissions.push(match (mode & 0o100 != 0, mode & 0o4000 != 0) {
        (true, true) => 's',  // Execute + setuid
        (false, true) => 'S', // Setuid without execute
        (true, false) => 'x',
        (false, false) => '-',
    });

    // group permissions
    permissions.push(if mode & 0o040 != 0 { 'r' } else { '-' });
    permissions.push(if mode & 0o020 != 0 { 'w' } else { '-' });
    permissions.push(match (mode & 0o010 != 0, mode & 0o2000 != 0) {
        (true, true) => 's',  // Execute + setgid
        (false, true) => 'S', // Setgid without execute
        (true, false) => 'x',
        (false, false) => '-',
    });

    // Other permissions
    permissions.push(if mode & 0o004 != 0 { 'r' } else { '-' });
    permissions.push(if mode & 0o002 != 0 { 'w' } else { '-' });
    permissions.push(match (mode & 0o001 != 0, mode & 0o1000 != 0) {
        (true, true) => 't',  // Execute + sticky
        (false, true) => 'T', // Sticky without execute
        (true, false) => 'x',
        (false, false) => '-',
    });

    permissions
}

fn get_file_type(metadata: &Metadata) -> char {
    let file_type = metadata.file_type();

    if file_type.is_dir() {
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
    }
}
