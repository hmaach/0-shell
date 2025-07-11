use std::{
    fs::Metadata,
    os::unix::fs::{FileTypeExt, MetadataExt, PermissionsExt},
    path::PathBuf,
};

pub fn get_permissions(metadata: &Metadata, path: &PathBuf) -> String {
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

    if has_acl(path) {
        permissions.push('+');
    }

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

fn has_acl(path: &PathBuf) -> bool {
    use std::ffi::CString;
    use std::os::raw::c_char;

    unsafe extern "C" {
        fn getxattr(
            path: *const c_char,
            name: *const c_char,
            value: *mut std::ffi::c_void,
            size: usize,
        ) -> isize;
    }

    let path_str = match path.to_str() {
        Some(s) => s,
        None => return false,
    };

    let c_path = match CString::new(path_str) {
        Ok(p) => p,
        Err(_) => return false,
    };

    let acl_access = match CString::new("system.posix_acl_access") {
        Ok(attr) => attr,
        Err(_) => return false,
    };

    unsafe {
        // Check if the file has POSIX ACL extended attribute
        let size = getxattr(
            c_path.as_ptr(),
            acl_access.as_ptr(),
            std::ptr::null_mut(),
            0,
        );

        // If size > 0, ACLs are present
        // If size == -1, no ACLs or error
        size > 0
    }
}

pub fn get_major_minor(metadata: &Metadata) -> (u64, u64) {
    // Get the raw device number
    let dev = metadata.rdev();

    // Linux uses a specific bit layout for device numbers
    // Major number is in bits 8-19 and 32-63
    let major = ((dev >> 8) & 0xfff) | ((dev >> 32) & !0xfff);

    // Minor number is in bits 0-7 and 20-31
    let minor = (dev & 0xff) | ((dev >> 12) & !0xff);

    (major, minor)
}
