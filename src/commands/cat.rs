use std::{env, fs};
use std::io::stdin;

use crate::commands::Command;
use crate::error::*;

pub struct CatCommand;

impl Command for CatCommand {
    fn execute(&self, args: Vec<String>) -> Result<(), ShellError> {
        if args.is_empty() {
            loop {
                let mut input = String::new();
                if let Err(e) = stdin().read_line(&mut input) {
                    eprintln!("cat: error reading the input: {}", e);
                    continue;
                }

                print!("{}", &input);
            }
        } else {
            let cur_dir = env::current_dir().map_err(|e| {
                ShellError::Other(format!("cat: failed to get current directory: {}", e))
            })?;

            for arg in args {
                let path = cur_dir.join(&arg);
                if path.is_file() {
                    let contents = match fs::read_to_string(path) {
                        Ok(c) => c,
                        Err(e) => {
                            eprintln!("cat: error reading file content '{}': {}", arg, e);
                            continue;
                        }
                    };

                    println!("{}%", contents);
                } else if path.is_dir() {
                    eprintln!("cat: src: Is a directory");
                } else {
                    eprintln!("cat: {}: No such file or directory", arg);
                }
            }
        }
        Ok(())
    }
}
