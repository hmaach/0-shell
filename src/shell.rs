use std::{
    collections::HashMap,
    io::{Write, stdin, stdout},
};

use crate::utils;
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
    }

    pub fn run_loop(&mut self) {
        println!("welcome to 01-shell");

        loop {
            print!("$ ");
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
        let (cmd, args) = utils::parse_command(input);

        if cmd.is_empty() {
            return Ok(());
        }

        match self.commands.get(&cmd) {
            None => Err(ShellError::CommandNotFound(cmd)),
            Some(command) => match command.execute(args) {
                Ok(()) => Ok(()),
                Err(err) => Err(err)
            },
        }
    }
}
