use std::{
    collections::HashMap, env::current_dir, io::{stdin, stdout, Write}, path::PathBuf
};

use crate::{parser, utils::{print_cur_dir, print_welcome}};
use crate::{
    commands::{Command, *},
    error::ShellError,
};

pub struct Shell {
    commands: HashMap<String, Box<dyn Command>>,
}

impl Shell {
    pub fn new() -> Self {
        let mut shell = Self {
            commands: HashMap::new(),
        };

        shell.register_commands();
        shell
    }

    fn register_commands(&mut self) {
        self.commands.insert("exit".to_owned(), Box::new(ExitCommand));
        self.commands.insert("pwd".to_owned(), Box::new(PwdCommand));
        self.commands.insert("echo".to_owned(), Box::new(EchoCommand));
        self.commands.insert("mkdir".to_owned(), Box::new(MkdirCommand));
        self.commands.insert("cd".to_owned(), Box::new(CdCommand));
        self.commands.insert("ls".to_owned(), Box::new(LsCommand));
        self.commands.insert("rm".to_owned(), Box::new(RmCommand));
        self.commands.insert("mv".to_owned(), Box::new(MvCommand));
        self.commands.insert("cp".to_owned(), Box::new(CpCommand));
        self.commands.insert("cat".to_owned(), Box::new(CatCommand));
    }

    pub fn run_loop(&mut self) {
        print_welcome();

        loop {
            let path: PathBuf = current_dir().expect("error getting path");

            print_cur_dir(path);
            
            stdout().flush().expect("error flush stdout");

            let mut input = String::new();
            match stdin().read_line(&mut input) {
                Ok(0) => {
                    println!("CTRL + D exit...");
                    break;
                }
                Ok(_) => {
                    input.pop();

                    match self.execute_command(input) {
                        Err(err) => println!("{}", err),
                        _ => ()
                    }
                }
                Err(error) => {
                    println!("ERROR: {}", error)
                }
            };
        }
    }

    fn execute_command(&mut self, input: String) -> Result<(), ShellError> {
        let (cmd, args) = parser::parse_command(input)?;

        if cmd.is_empty() {
            return Ok(());
        }

        match self.commands.get(&cmd) {
            None => Err(ShellError::CommandNotFound(cmd)),
            Some(command) => command.execute(args),
        }
    }
}
