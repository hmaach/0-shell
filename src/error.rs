use std::error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum ShellError {
    IoError(std::io::Error),
    CommandNotFound(String),
    Backticks,
    ArgsNotFound(String),
    Other(String),
}

impl fmt::Display for ShellError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ShellError::IoError(err) => write!(f, "{}", err),
            ShellError::CommandNotFound(cmd) => write!(f, "command '{}' not found", cmd),
            ShellError::Backticks => write!(f, "command substitution with backticks (`) is not supported in our mini shell"),
            ShellError::ArgsNotFound(cmd) => write!(f, "{}: missing file operand", cmd),
            ShellError::Other(err) => write!(f, "{}", err)
        }
    }
}

impl error::Error for ShellError {}

impl From<io::Error> for ShellError {
    fn from(err: io::Error) -> Self {
        ShellError::IoError(err)
    }
}
