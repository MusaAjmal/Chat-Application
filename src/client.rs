use std::io::{self, BufRead};

use actix_web::web::Bytes;
use awc::{Client, error::WsClientError, ws};
use futures_util::{SinkExt as _, StreamExt as _};
use tokio::select;
use tokio_stream::wrappers::UnboundedReceiverStream;
use tokio::sync::mpsc;

#[actix_web::main]
async fn main() -> Result<(), WsClientError> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting echo WebSocket client");

    let (cmd_tx, cmd_rx) = mpsc::unbounded_channel();
    let mut cmd_rx = UnboundedReceiverStream::new(cmd_rx);

    // spawn a thread to read lines from stdin
    std::thread::spawn(move || {
        let stdin = io::stdin();
        let reader = io::BufReader::new(stdin);

        for line in reader.lines() {
            match line {
                Ok(cmd) => {
                    if cmd_tx.send(cmd).is_err() {
                        log::error!("receiver dropped");
                        break;
                    }
                }
                Err(e) => {
                    log::error!("error reading stdin: {}", e);
                    break;
                }
            }
        }
    });

    let (response, mut connection) = Client::new()
        .ws("ws://127.0.0.1:8080/echo")
        .connect()
        .await?;

    log::debug!("Connected: {:?}", response);
    log::info!("Connected; server will echo messages sent");

    loop {
        select! {
            msg = connection.next() => {
                match msg {
                    Some(Ok(ws::Frame::Text(txt))) => {
                        log::info!("Server: {:?}", txt);
                    }
                    Some(Ok(ws::Frame::Ping(_))) => {
                        connection.send(ws::Message::Pong(Bytes::new())).await?;
                    }
                    Some(_) => {}
                    None => break,
                }
            }

            cmd = cmd_rx.next() => {
                if let Some(cmd) = cmd {
                    if cmd.is_empty() {
                        continue;
                    }
                    connection.send(ws::Message::Text(cmd.into())).await?;
                } else {
                    break;
                }
            }
        }
    }

    Ok(())
}
