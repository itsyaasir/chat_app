use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use crate::state::CLIENT_TIMEOUT;
use futures_util::{SinkExt, StreamExt};
use rand::Rng;
use tokio::{net::TcpStream, time::interval};
use tokio_tungstenite::accept_async;
use tungstenite::Message as WMessage;

use crate::state::{Appstate, ConnectionState};

pub async fn handle_connection(stream: TcpStream, state: Arc<Appstate>) -> anyhow::Result<()> {
    let (ws_sender, mut ws_receiver) = accept_async(stream).await?.split(); // Split the stream into a sender and receiver

    let mut new_msg_receiver = state.msg.subscribe(); // Subscribe to the message channel
    let mut connection_state = ConnectionState {
        connection_id: gen_random_user(),
        last_hbt: Instant::now(),
        ws_sender,
        msg: state.msg.clone(),
    }; // Create a new connection state

    // Send history message
    for old_chat in &*state.chat_history.read().await {
        connection_state
            .ws_sender
            .send(WMessage::Text(old_chat.to_string()))
            .await?;
    }

    let mut interval = interval(Duration::from_secs(1));

    loop {
        tokio::select! {
            msg = ws_receiver.next() => {
                if let Some(msg) = msg {
                    match msg? {
                        WMessage::Text(msg) => connection_state.handle_message(msg,&state.chat_history).await?,

                        WMessage::Ping(msg) => {
                            connection_state.ws_sender.send(WMessage::Pong(msg)).await?;
                        }

                        WMessage::Pong(_) => {
                            connection_state.last_hbt = Instant::now();
                        }
                        WMessage::Close(_) => break Ok(()),

                        WMessage::Binary(_) | WMessage::Frame(_) => unreachable!(),

                    }

                } else {
                    break Ok(());
                }
            }

            Ok(msg) = new_msg_receiver.recv() => {
                connection_state.ws_sender.send(WMessage::Text(msg.to_string())).await?;
            }

            _ = interval.tick() => {
                if connection_state.last_hbt.elapsed().as_secs() > CLIENT_TIMEOUT.as_secs() {
                log::info!("server close dead connection");
                break Ok(());
                }
                connection_state.ws_sender.send(WMessage::Ping(Vec::new())).await?;
            }

        }
    }
}

pub fn gen_random_user() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..=100000000)
}
