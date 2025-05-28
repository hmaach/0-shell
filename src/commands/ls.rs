use std::fs;

use crate::commands::Command;
use crate::error::*;

pub struct LsCommand;

impl Command for LsCommand {
    fn execute(&self, args: Vec<String>) -> Result<(), ShellError> {
        let dir = "./";
        let a_flag = args.contains(&"-a".to_string());
        let f_flag = args.contains(&"-F".to_string());

        let mut cleaned_paths: Vec<String> = Vec::new();

        if a_flag { // hard coding hhhhhhh
            cleaned_paths.push(if f_flag { "./".to_string() } else { ".".to_string() });
            cleaned_paths.push(if f_flag { "../".to_string() } else { "..".to_string() });
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

                    let mut str_name = path.file_name()?.to_str()?.to_string();

                    if f_flag && path.is_dir()  {
                        str_name.push('/');
                    }

                    Some(str_name)
                })
        );

        cleaned_paths.sort();

        for path in cleaned_paths {
            print!("{}  ", path);
        }
        println!();

        Ok(())
    }
}
