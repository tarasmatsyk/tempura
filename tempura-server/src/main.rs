mod server;
mod stream;

use pico_args::Arguments;
use std::io;
use thiserror::Error;

const DEFAULT_ADDRESS: &str = "127.0.0.1:8025";

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

    let addr: String = pargs
        .opt_value_from_str(["-b", "--bind"])
        .map_err(AppError::Arg)?
        .unwrap_or_else(|| DEFAULT_ADDRESS.to_string());

    server::run(&addr).await.map_err(AppError::Io)
}
