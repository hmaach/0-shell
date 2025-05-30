pub fn parse_command(input: String) -> (String, Vec<String>) {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    if parts.is_empty() {
        return (String::new(), Vec::new());
    }

    let cmd = parts[0].to_string();
    let args = parts[1..].iter().map(|arg| arg.to_string()).collect();

    (cmd, args)
}
