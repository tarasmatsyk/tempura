use std::io;
use thiserror::Error;
use tokio::net::TcpStream;
use tokio::io::{BufReader, AsyncWriteExt, AsyncBufReadExt};


pub async fn listen(socket: TcpStream) -> io::Result<()> {
    let (reader, mut writer) = socket.into_split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    writer.write_all(b"220 localhost ESMTP fake-server\r\n").await?;

    let mut data_mode = false;
    let mut email_data = String::new();

    loop {
        line.clear();
        let bytes = reader.read_line(&mut line).await?;
        if bytes == 0 {
            break;
        }

        let input = line.trim_end();

        if data_mode {
            if input == "." {
                // End of DATA
                println!("\n=== RECEIVED EMAIL ===\n{}\n======================\n", email_data);
                email_data.clear();
                data_mode = false;
                writer.write_all(b"250 OK\r\n").await?;
            } else {
                email_data.push_str(input);
                email_data.push('\n');
            }
            continue;
        }

        match input {
            cmd if cmd.starts_with("HELO") || cmd.starts_with("EHLO") => {
                writer.write_all(b"250 Hello\r\n").await?;
            }
            cmd if cmd.starts_with("MAIL FROM") => {
                writer.write_all(b"250 OK\r\n").await?;
            }
            cmd if cmd.starts_with("RCPT TO") => {
                writer.write_all(b"250 OK\r\n").await?;
            }
            "DATA" => {
                writer.write_all(b"354 End data with <CR><LF>.<CR><LF>\r\n").await?;
                data_mode = true;
            }
            "QUIT" => {
                writer.write_all(b"221 Bye\r\n").await?;
                break;
            }
            _ => {
                writer.write_all(b"250 OK\r\n").await?;
            }
        }
    }

    Ok(())
}
