pub fn run_shell() {
    loop {
        print!("$ ");
        use std::io::{ self, Write };
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() || input.trim().is_empty() {
            break;
        }

        println!("0-shell: command not found (yet): {}", input.trim());
    }
}
