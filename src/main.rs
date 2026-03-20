mod server;

use pico_args::Arguments;
use std::io;
use thiserror::Error;
use tokio::net::TcpListener;

const DEFAULT_ADDRESS: &str = "127.0.0.1:8025";

use server::listen;

#[derive(Debug, Error)]
enum AppError {
    #[error("Couldn't bind listener: {0}")]
    Io(#[from] io::Error),

    #[error("Argument parsing failed: {0}")]
    Arg(#[from] pico_args::Error),
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let mut pargs = Arguments::from_env();

    let bind: String = pargs
        .opt_value_from_str(["-b", "--bind"])
        .map_err(AppError::Arg)?
        .unwrap_or_else(|| DEFAULT_ADDRESS.to_string());

    let listener = TcpListener::bind(&bind).await.map_err(AppError::Io)?;
    println!("SMTP server listening on {}", bind);

    loop {
        let (socket, peer) = listener.accept().await?;
        println!("New connection from {}", peer);

        tokio::spawn(async move {
            if let Err(e) = listen(socket).await {
                eprintln!("Connection error: {}", e);
            }
        });
    }
}
