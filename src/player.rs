use std::collections::VecDeque;

use anyhow::Context;
use axum::extract::ws::{Message, WebSocket};
use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use snarkvm::prelude::*;

use crate::{
    table::Status,
};

#[derive(Debug)]
pub struct Player<N: Network> {
    address: Address<N>,
    sink: SplitSink<WebSocket, Message>,
    stream: SplitStream<WebSocket>,
}

impl<N: Network> Player<N> {
    pub fn new(address: Address<N>, socket: WebSocket) -> Self {
        let (sink, stream) = socket.split();
        Self {
            address,
            sink,
            stream,
        }
    }

    pub fn address(&self) -> &Address<N> {
        &self.address
    }

    pub async fn notify(&mut self, message: Message) -> Result<()> {
        self.sink.send(message).await.context("notify")
    }

    pub async fn recv(&mut self) -> Result<Option<Message>> {
        self.stream.next().await.transpose().context("recv")
    }

    pub async fn recv_request(&mut self) -> Result<VecDeque<Request<N>>> {
        let msg = self.stream.next().await.transpose().context("recv")?;
        let request = match msg {
            Some(Message::Text(msg)) => serde_json::from_str(&msg)?,
            None => bail!("Disconnect"),
            _ => bail!("Not follow protocol"),
        };
        Ok(request)
    }

    pub async fn notify_start(&mut self, id: &str, address: &Address<N>) -> Result<()> {
        self.notify(Message::Text(serde_json::to_string(&(id, address))?)).await
    }

    pub async fn notify_status(&mut self, status: Status, response: Response<N>) -> Result<()> {
        let mut records = vec![];
        response.outputs().iter().for_each(|val| {
            if let Value::Record(record) = val {
                let owner: Address<N> = *record.owner().clone();
                if self.address == owner {
                    records.push(serde_json::to_string(&record).unwrap());
                }
            };
        });
        self.notify(Message::Text(serde_json::to_string_pretty(&(status, records))?)).await
    }
}
