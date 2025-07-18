use chrono::{DateTime, Duration, TimeZone, Utc};
use chrono_tz::Africa::Casablanca;

use std::{
    ffi::CStr,
    fs::Metadata,
    os::unix::fs::{FileTypeExt, MetadataExt},
    path::PathBuf,
};

use super::{file_permissions::get_permissions, formatter::format_path, parser::Flag};

use crate::{
    commands::ls::{file_permissions::get_major_minor, formatter::quote_if_needed},
    error::ShellError,
};

pub fn get_detailed_file_info(
    path: &PathBuf,
    total_blocks: Option<&mut u64>,
    max_len: &mut usize,
    flags: &Flag,
) -> Result<Vec<String>, ShellError> {
    let metadata = path.symlink_metadata()?;

    let permission = get_permissions(&metadata, &path);

    let size = if metadata.file_type().is_char_device() || metadata.file_type().is_block_device() {
        let (major, minor) = get_major_minor(&metadata);
        let mut res = String::new();

        res.push_str(&major.to_string());
        res.push_str(", ");
        res.push_str(&minor.to_string());
        res
    } else {
        metadata.len().to_string()
    };

    if size.len() > *max_len {
        *max_len = size.len();
    }

    let mut file_name = path
        .file_name()
        .and_then(|s| s.to_str())
        .map(|s| s.to_string())
        .or_else(|| Some(path.to_string_lossy().to_string()))
        .ok_or_else(|| {
            ShellError::Other(format!("Unable to get file name for path: {:?}", path))
        })?;

    quote_if_needed(&mut file_name);
    format_path(path, &mut file_name, flags)?;

    let (user_owner, group_owner) = get_owners_info(&metadata)
        .map_err(|e| ShellError::Other(format!("cannot access '{}': {}", path.display(), e)))?;

    let n_link = metadata.nlink().to_string();

    let modified_at = get_modified_at(&metadata);

    if let Some(blocks) = total_blocks {
        *blocks += metadata.blocks() / 2;
    }

    Ok(vec![
        permission,
        n_link,
        user_owner,
        group_owner,
        size,
        modified_at,
        file_name,
    ])
}

fn get_modified_at(metadata: &Metadata) -> String {
    match metadata.modified() {
        Ok(modified_at) => {
            let datetime_utc: DateTime<Utc> = modified_at.into();

            let datetime = Casablanca.from_utc_datetime(&datetime_utc.naive_utc());

            let now = Casablanca.from_utc_datetime(&Utc::now().naive_utc());
            let six_months_ago = now - Duration::days(30 * 6);

            if datetime > six_months_ago {
                datetime.format("%b %e %H:%M").to_string()
            } else {
                datetime.format("%b %e  %Y").to_string()
            }
        }
        Err(_) => "<invalid time>".to_string(),
    }
}

fn get_owners_info(metadata: &Metadata) -> Result<(String, String), ShellError> {
    let uid = metadata.uid();
    let gid = metadata.gid();

    unsafe {
        let passwd = libc::getpwuid(uid);

        let username = if !passwd.is_null() {
            CStr::from_ptr((*passwd).pw_name)
                .to_str()
                .map_err(|_| {
                    ShellError::Other(format!("Invalid UTF-8 in group name for UID({})", uid))
                })?
                .to_string()
        } else {
            uid.to_string()
        };

        let group = libc::getgrgid(gid);
        let groupname = if !group.is_null() {
            CStr::from_ptr((*group).gr_name)
                .to_str()
                .map_err(|_| {
                    ShellError::Other(format!("Invalid UTF-8 in group name for GID({})", gid))
                })?
                .to_string()
        } else {
            gid.to_string()
        };

        Ok((username, groupname))
    }
}
