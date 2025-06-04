use std::path::PathBuf;

pub enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    fn to_code(&self) -> u8 {
        match self {
            Color::Red => 31,
            Color::Green => 32,
            Color::Blue => 34,
        }
    }
}

pub fn colorize(text: &str, color: Color, bold: bool) -> String {
    let bold_prefix = if bold { "1;" } else { "" };
    format!("\x1b[{}{}m{}\x1b[0m", bold_prefix, color.to_code(), text)
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
    println!("\t\t      {}", colorize("Welcome to", Color::Blue, false));
    println!("{}", colorize(title, Color::Blue, true));
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
