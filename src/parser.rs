use crate::error::ShellError;

pub fn parse_command(input: String) -> Result<(String, Vec<String>), ShellError> {
    let input = input.trim();

    if input.is_empty() {
        return Ok((String::new(), Vec::new()));
    }

    if input.contains('`') {
        return Err(ShellError::Backticks);
    }

    let parts = parser(input)?;

    if parts.is_empty() {
        return Ok((String::new(), Vec::new()));
    }

    let cmd = parts[0].to_string();
    let args = parts[1..].iter().map(|arg| arg.to_string()).collect();

    Ok((cmd, args))
}

fn parser(input: &str) -> Result<Vec<&str>, ShellError> {
    Ok(input.split_ascii_whitespace().collect())
}