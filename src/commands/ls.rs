use std::fs;
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
    // dbg!(get_permission_string1(path));
    // dbg!(path.metadata().unwrap().permissions());
    let permission = get_permission_string(path);
    result.push_str(permission.as_str());
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

// fn get_permission_string1(path: &PathBuf) -> Option<String> {
//     let output = process::Command::new("ls")
//         .arg("-ld")
//         .arg(path)
//         .output()
//         .ok()?;

//     if output.status.success() {
//         let stdout = String::from_utf8_lossy(&output.stdout);
//         // First word is like -rw-r--r--
//         let perms = stdout.split_whitespace().next()?.to_string();
//         Some(perms)
//     } else {
//         None
//     }
// }
