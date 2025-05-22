use std::io::{ stdin, stdout, Write };

pub fn run_shell() {
    loop {
        print!("$ ");
        stdout().flush().unwrap();

        let mut input = String::new();
        if stdin().read_line(&mut input).is_err() {
            break;
        }

        // if the command is empty do noting
        if input.trim().is_empty() {
            continue;
        }

        println!("0-shell: command not found (yet): {}", input.trim());
    }
}
