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
        }
        Ok(())
    }
}
