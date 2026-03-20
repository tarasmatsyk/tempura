use std::io;
use tokio::net::TcpListener;
use crate::stream::listen;

pub async fn run(addr: &str) -> Result<(), io::Error> {
    let l = TcpListener::bind(addr).await?;
    println!("SMTP server listening on {}", addr);
    loop {
        let (socket, peer) = l.accept().await?;
        // TODO: add logger
        println!("New connection from {}", peer);

        tokio::spawn(async move {
            if let Err(e) = listen(socket).await {
                println!("Connection error: {}", e);
            }
        });
    }

}
