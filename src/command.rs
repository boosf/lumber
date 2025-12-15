use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseCommandError {
    #[error("invalid command {command}")]
    InvalidCommand { command: String },
}

#[derive(Debug)]
pub enum Command {
    Send = 1,
    Get = 2,
}

impl FromStr for Command {
    type Err = ParseCommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Command::Send),
            "2" => Ok(Command::Get),
            _ => Err(ParseCommandError::InvalidCommand {
                command: s.to_string(),
            }),
        }
    }
}
