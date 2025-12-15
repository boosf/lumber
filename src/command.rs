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

#[derive(Debug, Error)]
pub enum ExecuteError {}

#[derive(Debug)]
pub enum ExecuteStatus {
    Success = 0,
    Failed = 1,
}

#[derive(Debug)]
pub struct ExecuteResult {
    pub messages: Vec<Message>,
    pub status: ExecuteStatus,
}

#[derive(Debug)]
pub struct Message {
    pub key: String,
    pub partition: String,
    pub offset: i64,
    pub body: Vec<u8>,
}

pub trait CommandExecutor {
    fn execute(&self) -> Result<ExecuteResult, ExecuteError>;
}

#[derive(Debug)]
pub struct CommandExecutorFactory {
    body: String,
    command: Command,
}

impl CommandExecutorFactory {
    pub fn new(command: Command, body: String) -> Self {
        Self { body, command }
    }
}

impl CommandExecutor for CommandExecutorFactory {
    fn execute(&self) -> Result<ExecuteResult, ExecuteError> {
        return match self.command {
            Command::Send => {
                let executor = SendCommandExecutor::new(&self.body);
                executor.execute()
            }
            Command::Get => {
                let executor = GetCommandExecutor::new(&self.body);
                executor.execute()
            }
        };
    }
}

#[derive(Debug)]
struct SendCommandExecutor<'a> {
    body: &'a String,
}

impl<'a> SendCommandExecutor<'a> {
    fn new(body: &'a String) -> Self {
        Self { body }
    }
}

impl<'a> CommandExecutor for SendCommandExecutor<'a> {
    fn execute(&self) -> Result<ExecuteResult, ExecuteError> {
        println!("This is the send body {}", self.body);
        Ok(ExecuteResult {
            messages: vec![],
            status: ExecuteStatus::Success,
        })
    }
}

#[derive(Debug)]
struct GetCommandExecutor<'a> {
    body: &'a String,
}

impl<'a> GetCommandExecutor<'a> {
    fn new(body: &'a String) -> Self {
        Self { body }
    }
}

impl<'a> CommandExecutor for GetCommandExecutor<'a> {
    fn execute(&self) -> Result<ExecuteResult, ExecuteError> {
        println!("This is the get body {}", self.body);
        Ok(ExecuteResult {
            messages: vec![Message {
                body: Vec::new(),
                key: "".to_string(),
                offset: 1,
                partition: "3".to_string(),
            }],
            status: ExecuteStatus::Success,
        })
    }
}
