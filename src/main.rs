mod client;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

const MAX_BUF_SIZE: usize = 1024;
const MAX_HEADER_SIZE: usize = 10;
const DELIM: char = ';';

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).await?;

    loop {
        let (mut socket, addr) = listener.accept().await?;
        tokio::spawn(handle_client(socket));
    }
}

async fn handle_client(mut socket: TcpStream) -> tokio::io::Result<()> {
    let mut out: Option<String> = None;
    let mut buf = [0; MAX_BUF_SIZE];

    if let Ok(n) = socket.read(&mut buf).await {
        let message = String::from_utf8_lossy(&buf[..n]);
        if out.is_none() {
            let mut msg_size = String::with_capacity(MAX_HEADER_SIZE);
            for c in message.chars() {
                if c == DELIM {
                    let size = match msg_size.parse::<u32>() {
                        Ok(val) => Some(val),
                        _ => None,
                    };
                    break;
                }
                msg_size.push(c);
            }
        }
        let _ = socket.write_all(&buf[..n]).await;
    }
    Ok(())
}
