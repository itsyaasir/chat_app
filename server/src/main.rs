mod connection;
mod message;
mod state;

use std::sync::Arc;

use log::LevelFilter;
use tokio::{
    net::TcpListener,
    sync::{broadcast::channel, RwLock},
};

use crate::{connection::handle_connection, message::ChatMessage, state::Appstate};

const WS_ADDRESS: &str = "127.0.0.1:8080";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Logger
    env_logger::Builder::new()
        .filter_level(LevelFilter::Trace)
        .init();

    let listener = TcpListener::bind(WS_ADDRESS).await?;
    log::info!("Listening on: {}", WS_ADDRESS);

    let (msg, _) = channel::<ChatMessage>(5000);

    // App state
    let state = Arc::new(Appstate {
        chat_history: RwLock::new(Vec::new()),
        msg,
    });

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection(stream, state.clone()));
    }
    Ok(())
}
