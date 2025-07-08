use chrono::TimeZone;
use chrono::Utc;
use chrono::{DateTime, Duration};
use chrono_tz::Africa::Casablanca;
use std::{ffi::CStr, path::PathBuf};

use crate::commands::ls::formatter::format_path;
use crate::commands::ls::parser::Flag;
use crate::error::ShellError;
use std::fs::Metadata;
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;

pub fn get_detailed_file_info(
    path: &PathBuf,
    total_blocks: Option<&mut u64>,
    flags: &Flag,
) -> Result<Vec<String>, ShellError> {
    let metadata = path.symlink_metadata()?;

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

    let _ = format_path(path, &mut file_name, flags);

    // let file_name: String = match format_path(path, &mut file_name, flags) {
    //     Ok(_) => file_name,
    //     Err(_) => colorize(&file_name, Color::Red, false),
    // };

    let (owner_name, group_name) = get_file_owner_and_group(&metadata)
        .map_err(|e| ShellError::Other(format!("cannot access '{}': {}", path.display(), e)))?;

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

pub fn get_file_owner_and_group(metadata: &Metadata) -> Result<(String, String), ShellError> {
    let uid = metadata.uid();
    let gid = metadata.gid();

    unsafe {
        let passwd = libc::getpwuid(uid);
        if passwd.is_null() {
            return Err(ShellError::Other(format!(
                "Failed to get user name for UID({})",
                uid
            )));
        }
        let username = CStr::from_ptr((*passwd).pw_name)
            .to_str()
            .map_err(|_| ShellError::Other(format!("Invalid UTF-8 in user name for UID({})", uid)))?
            .to_string();

        let group = libc::getgrgid(gid);
        if group.is_null() {
            return Err(ShellError::Other(format!(
                "Failed to get group name for GID({})",
                gid
            )));
        }
        let groupname = CStr::from_ptr((*group).gr_name)
            .to_str()
            .map_err(|_| {
                ShellError::Other(format!("Invalid UTF-8 in group name for GID({})", gid))
            })?
            .to_string();

        Ok((username, groupname))
    }
}

pub fn get_modified_at(metadata: &Metadata) -> String {
    match metadata.modified() {
        Ok(modified_at) => {
            // Convert system time to UTC first
            let datetime_utc: DateTime<Utc> = modified_at.into();

            // Convert to Casablanca timezone
            let datetime = Casablanca.from_utc_datetime(&datetime_utc.naive_utc());

            // Get current time in Casablanca
            let now = Casablanca.from_utc_datetime(&Utc::now().naive_utc());
            let six_months_ago = now - Duration::days(30 * 6);

            // Compare and format
            if datetime > six_months_ago {
                datetime.format("%b %e %H:%M").to_string()
            } else {
                datetime.format("%b %e  %Y").to_string()
            }
        }
        Err(_) => "<invalid time>".to_string(),
    }
}
