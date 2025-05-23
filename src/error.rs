use std::error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum ShellError {
    IoError(std::io::Error),
    CommandNotFound(String),
    InvalidArguments(String),
}

impl fmt::Display for ShellError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ShellError::IoError(err) => write!(f, "{}", err),
            ShellError::CommandNotFound(cmd) => write!(f, "command '{}' not found", cmd),
            ShellError::InvalidArguments(arg) => write!(f, "invalid argument: {}", arg),
        }
    }
}

impl error::Error for ShellError {}

impl From<io::Error> for ShellError {
    fn from(err: io::Error) -> Self {
        ShellError::IoError(err)
    }
}
