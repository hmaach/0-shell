use std::ffi::CStr;
use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;

use crate::commands::Command;
use crate::error::*;
use crate::utils::{get_permission_string, sort_vector};
// use std::process;

pub struct LsCommand;

impl Command for LsCommand {
    fn execute(&self, args: Vec<String>) -> Result<(), ShellError> {
        let dir = "./";
        let a_flag = args.contains(&"-a".to_string());
        let f_flag = args.contains(&"-F".to_string());
        let l_flag = args.contains(&"-l".to_string());

        let mut cleaned_paths: Vec<String> = Vec::new();

        if a_flag {
            // hard coding hhhhhhh
            cleaned_paths.push(if f_flag {
                "./".to_string()
            } else {
                ".".to_string()
            });
            cleaned_paths.push(if f_flag {
                "../".to_string()
            } else {
                "..".to_string()
            });
        }

        cleaned_paths.extend(
            fs::read_dir(dir)
                .unwrap()
                .filter_map(Result::ok)
                .filter_map(|entry| {
                    let path = entry.path();

                    if !a_flag {
                        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                            if name.starts_with('.') {
                                return None;
                            }
                        }
                    }

                    let mut str_name = String::new();
                    if l_flag {
                        str_name.push_str(format_long_format(&path).as_str());
                        str_name.push_str("\n");
                    } else {
                        str_name.push_str(path.file_name()?.to_str().unwrap());

                        if f_flag && path.is_dir() {
                            str_name.push('/');
                        }
                        str_name.push_str("  ");
                    }

                    Some(str_name)
                }),
        );

        print(&mut cleaned_paths);

        Ok(())
    }
}

fn format_long_format(path: &PathBuf) -> String {
    let mut result = String::new();
    let permission = get_permission_string(path);
    let len = path.metadata().unwrap().len().to_string();
    let file_name = path.file_name().unwrap();
    let (owner_name, group_name) = get_file_owner_and_group(path);
    let n_link = path.metadata().unwrap().nlink().to_string();
    let created_at: std::time::SystemTime = path.metadata().unwrap().created().unwrap().;

    dbg!(created_at);

    result.push_str(permission.as_str());
    result.push_str(" ");
    result.push_str(n_link.as_str());
    result.push_str(" ");
    result.push_str(owner_name.as_str());
    result.push_str(" ");
    result.push_str(group_name.as_str());
    result.push_str(" ");
    result.push_str(len.as_str());
    result.push_str(" ");
    // result.push_str(created_at.to_str().unwrap());
    // result.push_str(" ");
    result.push_str(file_name.to_str().unwrap());

    result
}

fn print(result: &mut Vec<String>) {
    sort_vector(result);

    if let Some(mut last) = result.pop() {
        if last.ends_with("\n") {
            last.pop();
        } else if last.ends_with("  ") {
            last.pop();
            last.pop();
        }
        result.push(last);
    }

    for path in result {
        print!("{}", path);
    }
    println!();
}

fn get_file_owner_and_group(path: &PathBuf) -> (String, String) {
    let metadata = fs::metadata(path).expect("Failed to get metadata");
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
