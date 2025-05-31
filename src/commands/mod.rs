use crate::error::ShellError;

pub trait Command {
    fn execute(&self, args: Vec<String>) -> Result<(), ShellError>;
}

pub mod pwd;
pub mod cd;
pub mod ls;
pub mod mkdir;
pub mod cat;
pub mod cp;
pub mod mv;
pub mod rm;
pub mod echo;
pub mod exit;

pub use exit::ExitCommand;
pub use pwd::PwdCommand;
pub use echo::EchoCommand;
pub use mkdir::MkdirCommand;
pub use cd::CdCommand;
pub use ls::LsCommand;
pub use rm::RmCommand;
pub use mv::MvCommand;
pub use cp::CpCommand;
pub use cat::CatCommand;