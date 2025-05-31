mod commands;
mod error;
mod shell;
mod parser;

fn main() {
    let mut shell = shell::Shell::new();
    shell.run_loop();
}
