mod command;
mod parser;

use tokio::net::{TcpListener, TcpStream};

use crate::command::CommandExecutor;

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
    let body = parser::parse_body(&mut socket).await?;
    let executor = command::CommandExecutorFactory::new(body.command, body.message);
    match executor.execute() {
        Ok(res) => println!("Result: {:?}", res),
        Err(e) => eprintln!("Error: {}", e),
    }
    Ok(())
}
