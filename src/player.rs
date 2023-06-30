use std::{collections::VecDeque, sync::Arc};

use anyhow::Context;
use axum::extract::ws::{Message, WebSocket};
use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use snarkvm::prelude::*;
use tokio::sync::Mutex;

use crate::{
    table::Status,
};

#[derive(Clone, Debug)]
pub struct Player<N: Network> {
    address: Address<N>,
    sink: Arc<Mutex<SplitSink<WebSocket, Message>>>,
    stream: Arc<Mutex<SplitStream<WebSocket>>>,
}

#[derive(Serialize)]
pub enum PlayerMessage<N: Network> {
    Start(String, Address<N>),
    GameStatus(Status, Vec<Record<N, Plaintext<N>>>),
    TxID(N::TransactionID),
}

impl<N: Network> Player<N> {
    pub fn new(address: Address<N>, socket: WebSocket) -> Self {
        let (sink, stream) = socket.split();
        Self {
            address,
            sink: Arc::new(Mutex::new(sink)),
            stream: Arc::new(Mutex::new(stream)),
        }
    }

    pub fn address(&self) -> &Address<N> {
        &self.address
    }

    pub async fn notify(&self, message: PlayerMessage<N>) -> Result<()> {
        let message = Message::Text(serde_json::to_string(&message)?);
        let mut sink = self.sink.lock().await;
        sink.send(message).await.context("notify")
    }

    pub async fn recv(&self) -> Result<Option<Message>> {
        self.stream.lock().await.next().await.transpose().context("recv")
    }

    pub async fn recv_request(&self) -> Result<VecDeque<Request<N>>> {
        let msg = self.recv().await?;
        let request = match msg {
            Some(Message::Text(msg)) => serde_json::from_str(&msg)?,
            None => bail!("Disconnect"),
            _ => bail!("Not follow protocol"),
        };
        Ok(request)
    }

    pub async fn notify_start(&self, id: &str, address: &Address<N>) -> Result<()> {
        self.notify(PlayerMessage::Start(id.to_string(), *address)).await
    }

    pub async fn notify_status(&self, status: Status, response: Response<N>) -> Result<()> {
        let mut records = vec![];
        response.outputs().iter().for_each(|val| {
            if let Value::Record(record) = val {
                let owner: Address<N> = *record.owner().clone();
                if self.address == owner {
                    records.push(record.clone());
                }
            };
        });
        self.notify(PlayerMessage::GameStatus(status, records)).await
    }

    pub async fn notify_tx_id(&self, tx_id: N::TransactionID) -> Result<()> {
        self.notify(PlayerMessage::TxID(tx_id)).await
    }
}
