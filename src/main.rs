mod client;

use std::cmp;
use std::io::{self, ErrorKind};
use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream};

const MAX_BUF_SIZE: usize = 1024;
const MAX_HEADER_SIZE: usize = 10;
const DELIM: char = ';';

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).await?;

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(handle_client(socket));
    }
}

async fn handle_client(mut socket: TcpStream) -> tokio::io::Result<()> {
    let mut out: Option<String> = None;

    loop {
        let mut buf = [0; MAX_BUF_SIZE];
        let n = socket.read(&mut buf).await?;
        let message = String::from_utf8_lossy(&buf[..n]);

        let mut start: usize = 0;

        if out.is_none() {
            let (start_idx, size) = parse_header(&message)
                .map_err(|_| io::Error::new(ErrorKind::InvalidData, "failed to parse header"))?;
            start = start_idx;
            out = Some(String::with_capacity(size));
        }

        let str_buf = out.as_mut().unwrap();
        let remaining = str_buf.capacity() - str_buf.len();
        let end: usize = cmp::min(MAX_BUF_SIZE, remaining);

        for c in message.chars().skip(start).take(end) {
            str_buf.push(c);
        }

        if str_buf.capacity() == str_buf.len() {
            println!("This is the message: {}", str_buf);
            break;
        }
    }

    Ok(())
}

fn parse_header(msg: &str) -> Result<(usize, usize), ()> {
    let mut msg_size = String::with_capacity(MAX_HEADER_SIZE);
    for (idx, c) in msg.chars().enumerate() {
        if c == DELIM {
            let size = msg_size.parse::<usize>().map_err(|_| ())?;
            return Ok((idx + 1, size));
        }
        msg_size.push(c);
    }
    Err(())
}
