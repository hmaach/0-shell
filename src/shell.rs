use std::io::{ stdin, stdout, Write };

pub fn run_shell() {
    loop {
        print!("$ ");
        stdout().flush().unwrap();

        let mut input = String::new();
        if stdin().read_line(&mut input).is_err() {
            println!("Failed to read input");
            continue;
        }

        let trimmed = input.trim();

        if trimmed.is_empty() {
            continue;
        }

        // split command and args
        let mut parts = trimmed.split_whitespace();
        let command = parts.next().unwrap(); // take the fist part

        match command {
            "cat" => {
                println!("'cat' handler is not implemented yet !");
            }
            "cd" => {
                println!("'cd' handler is not implemented yet !");
            }
            "cp" => {
                println!("'cp' handler is not implemented yet !");
            }
            "echo" => {
                println!("'echo' handler is not implemented yet !");
            }
            "ls" => {
                println!("'ls' handler is not implemented yet !");
            }
            "mkdir" => {
                println!("'mkdir' handler is not implemented yet !");
            }
            "mv" => {
                println!("'mv' handler is not implemented yet !");
            }
            "pwd" => {
                println!("'pwd' handler is not implemented yet !");
            }
            "rm" => {
                println!("'rm' handler is not implemented yet !");
            }
            "exit" => {
                println!("Exited seccafully !");
                break;
            }
            _ => {
                println!("0-shell: command not found : {}", command);
            }
        }
    }
}
