use std::path::PathBuf;

use colored::{ColoredString, Colorize};

pub enum Color {
    Red,
    Orange,
    Green,
    Blue,
    SkyBlue,
}

pub fn colorize(text: &str, color: Color, bold: bool) -> ColoredString {
    let result = match color {
        Color::Red => text.red(),
        Color::Green => text.green(),
        Color::Blue => text.blue(),
        Color::Orange => text.truecolor(255, 165, 0),
        Color::SkyBlue => text.truecolor(135, 206, 235),
    };

    if bold { result.bold() } else { result }
}

pub fn clean_string(s: String) -> String {
    s.chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
        .to_uppercase()
}

// I get it from here "https://patorjk.com/software/taag"
pub fn print_welcome() {
    let title = r#"
 ██████╗       ███████╗██╗  ██╗███████╗██╗     ██╗     
██╔═████╗      ██╔════╝██║  ██║██╔════╝██║     ██║     
██║██╔██║█████╗███████╗███████║█████╗  ██║     ██║     
████╔╝██║╚════╝╚════██║██╔══██║██╔══╝  ██║     ██║     
╚██████╔╝      ███████║██║  ██║███████╗███████╗███████╗
 ╚═════╝       ╚══════╝╚═╝  ╚═╝╚══════╝╚══════╝╚══════╝
"#;

    println!();
    println!("\t\t      {}", colorize("Welcome to", Color::Orange, false));
    println!("{}", colorize(title, Color::Orange, true));
    println!(
        "\t   {} {} {}",
        colorize("Yassine El Mach", Color::Green, true),
        colorize("&&", Color::Red, true),
        colorize("Hamza Maach", Color::Green, true)
    );
    println!();
}

pub fn print_cur_dir(path: PathBuf) {
    let dir_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("~");

    let prompt = format!(
        "{} {} ",
        colorize(&format!("{dir_name}"), Color::Blue, true),
        colorize("➤", Color::Red, true)
    );

    print!("{}", prompt);
}
