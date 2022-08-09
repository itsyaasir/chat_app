use std::time::{Duration, Instant};

use futures_util::stream::SplitSink;
use tokio::{
    net::TcpStream,
    sync::{broadcast::Sender, RwLock},
};
use tokio_tungstenite::WebSocketStream;
use tungstenite::Message as WMessage;

use crate::message::ChatMessage;

/// How long before lack of client response causes a timeout
pub const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct Appstate {
    pub chat_history: RwLock<Vec<ChatMessage>>,
    pub msg: Sender<ChatMessage>,
}

pub struct ConnectionState {
    pub connection_id: i32,
    pub last_hbt: Instant,
    pub ws_sender: SplitSink<WebSocketStream<TcpStream>, WMessage>,
    pub msg: Sender<ChatMessage>,
}

impl ConnectionState {
    pub async fn handle_message(
        &mut self,
        msg: String,
        chat_history: &RwLock<Vec<ChatMessage>>,
    ) -> anyhow::Result<()> {
        let message = ChatMessage {
            user_id: self.connection_id,
            message: msg.clone(),
            created_at: chrono::Local::now().naive_local(),
        };

        self.msg.send(message.clone())?;
        // Push history chat
        chat_history.write().await.push(message);

        Ok(())
    }
}
