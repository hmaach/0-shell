use std::io::{Write, stdin, stdout};

fn main() {
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

                parse_command(input);
            }
            Err(error) => {
                println!("error am3elem {}", error)
            }
        };
    }
}

fn parse_command(input: String) {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    if parts.is_empty() {
        return;
    }

    match parts[0] {
        "exit" => {
            println!("Exiting shell...");
            std::process::exit(0);
        }
        _ => {
            println!("Unknown command: {}", parts[0]);
        }
    }
}
