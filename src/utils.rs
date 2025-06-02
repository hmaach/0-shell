pub fn parse_command(input: String) -> (String, Vec<String>) {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    if parts.is_empty() {
        return (String::new(), Vec::new());
    }

    let cmd = parts[0].to_string();
    let args = parts[1..].iter().map(|arg| arg.to_string()).collect();

    (cmd, args)
}

pub fn clean_string(s: String) -> String {
    s.chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
        .to_uppercase()
}

pub fn colorize(text: &str, color: &str, bold: bool) -> String {
    // Map color names to ANSI color codes
    let color_code = match color.to_lowercase().as_str() {
        "red" => 31,
        "green" => 32,
        "blue" => 34,
        _ => 37,
    };

    let bold_code = if bold { "1;" } else { "" };

    // Format the string with ANSI escape codes
    format!("\x1b[{}{}m{}\x1b[0m", bold_code, color_code, text)
}
