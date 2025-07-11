mod commands;
mod error;
mod shell;
mod parser;
mod color;
mod utils;

fn main() {
    let mut shell = shell::Shell::new();
    shell.run_loop();
}
