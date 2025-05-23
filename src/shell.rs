pub enum ShellError {
    IoError(std::io::Error),
    CommandNotFound(String),
    InvalidArguments(String),
}

impl Display for ShellError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ShellError::IoError(err) => write!(f, "{}", err),
            ShellError::CommandNotFound(cmd) => write!(f, "command '{}' not found", cmd),
            ShellError::InvalidArguments(arg) => write!(f, "invalid argument: {}", arg),
        }
    }
}

impl std::error::Error for ShellError {}

impl From<std::io::Error> for ShellError {
    fn from(err: std::io::Error) -> Self {
        ShellError::IoError(err)
    }
}