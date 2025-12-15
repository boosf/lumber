use crate::command::{self};
use std::cmp;
use std::io::{self, ErrorKind};
use std::num::ParseIntError;
use thiserror::Error;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

const DELIM: char = ',';
const HEADER_END: char = ';';
const MAX_BUF_SIZE: usize = 1024;

#[derive(Debug)]
pub struct Body {
    message: String,
    command: command::Command,
}

pub async fn parse_body(socket: &mut TcpStream) -> tokio::io::Result<Body> {
    let mut out: Option<String> = None;
    let mut command: Option<command::Command> = None;
    loop {
        let mut buf = [0; MAX_BUF_SIZE];
        let n = socket.read(&mut buf).await?;
        let message = String::from_utf8_lossy(&buf[..n]);

        let mut start: usize = 0;

        if out.is_none() {
            let header = parse_header(&message).map_err(|e| {
                io::Error::new(
                    ErrorKind::InvalidData,
                    format!("failed to parse header, error: {}", e),
                )
            })?;
            start = header.start_idx;
            command = Some(header.command);
            out = Some(String::with_capacity(header.msg_size));
        }

        let str_buf = out.as_mut().unwrap();
        let remaining = str_buf.capacity() - str_buf.len();
        let end: usize = cmp::min(MAX_BUF_SIZE, remaining);

        for c in message.chars().skip(start).take(end) {
            str_buf.push(c);
        }

        if str_buf.capacity() == str_buf.len() {
            let message = out.take().unwrap();
            let command = command.take().unwrap();
            return Ok(Body {
                message,
                command: command,
            });
        }
    }
}

#[derive(Debug, Error)]
enum ParseHeaderError {
    #[error("missing header end")]
    MissingHeaderEnd,

    #[error("missing size")]
    MissingSize,

    #[error("missing command")]
    MissingCommand,

    #[error("invalid size")]
    InvalidSize {
        #[source]
        source: ParseIntError,
    },

    #[error("invalid command")]
    InvalidCommand {
        #[source]
        source: command::ParseCommandError,
    },
}

#[derive(Debug)]
struct Header {
    start_idx: usize,
    msg_size: usize,
    command: command::Command,
}

fn parse_header(msg: &str) -> Result<Header, ParseHeaderError> {
    let mut msg_splt = msg.split(HEADER_END);
    let header = msg_splt.next().ok_or(ParseHeaderError::MissingHeaderEnd)?;
    let mut header_splt = header.split(DELIM);

    let size_str = header_splt.next().ok_or(ParseHeaderError::MissingSize)?;
    let cmd_str = header_splt.next().ok_or(ParseHeaderError::MissingCommand)?;

    let msg_size = size_str
        .parse::<usize>()
        .map_err(|e| ParseHeaderError::InvalidSize { source: e })?;
    let command = cmd_str
        .parse::<command::Command>()
        .map_err(|e| ParseHeaderError::InvalidCommand { source: e })?;

    let start_idx = msg
        .find(HEADER_END)
        .ok_or(ParseHeaderError::MissingHeaderEnd)?;

    Ok(Header {
        start_idx: start_idx + 1,
        msg_size,
        command: command,
    })
}
