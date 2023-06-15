use std::net::SocketAddr;

use anyhow::Result;
use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use snarkvm::{prelude::*, synthesizer::helpers::memory::ConsensusMemory};
use tokio::sync::mpsc::{self, Sender};
use tracing::*;

use crate::{executor::Executor, player::Player, requests::action_from_request, table::ChessTable};

pub async fn run<N: Network, A: snarkvm::circuit::Aleo<Network = N>>() {
    let base_url = "http://192.168.200.25:3030";
    let query = Query::from(base_url);
    let storage = ConsensusMemory::open(None).unwrap();
    let executor = Executor::<N, _>::new(storage, query).unwrap();
    let tx = run_server::<N, A>(executor).await;

    let app = Router::new()
        .route("/battleship", get(ws_handler))
        .with_state(tx);

    info!("Server start");
    axum::Server::bind(&SocketAddr::from_str("0.0.0.0:3000").unwrap())
        .serve(app.into_make_service())
        .await
        .expect("couldn't start rest server");
}

async fn ws_handler<N: Network>(
    ws: WebSocketUpgrade,
    State(tx): State<Sender<Player<N>>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| async {
        if let Err(err) = handle_socket(socket, tx).await {
            info!("handle socket: {}", err);
        }
    })
}

async fn handle_socket<N: Network>(mut socket: WebSocket, tx: Sender<Player<N>>) -> Result<()> {
    let message = socket.recv().await;
    let message = match message {
        Some(message) => message?,
        None => {
            bail!("Connection broken");
        }
    };
    let address = match message {
        Message::Text(address) => Address::from_str(&address)?,
        _ => bail!("Not following protocol"),
    };
    tx.send(Player::new(address, socket)).await?;
    Ok(())
}

pub async fn run_server<N: Network, A: snarkvm::circuit::Aleo<Network = N>>(
    executor: Executor<N, ConsensusMemory<N>>,
) -> Sender<Player<N>> {
    let (tx, mut rx) = mpsc::channel(1024);
    let mut players = vec![];
    tokio::spawn(async move {
        loop {
            let player = rx.recv().await.unwrap();
            players.push(player);
            if players.len() == 2 {
                let executor = executor.clone();
                let player2 = players.pop().unwrap();
                let player1 = players.pop().unwrap();
                tokio::spawn(async move {
                        if let Err(err) = start_game::<N, A>(player1, player2, executor).await{
                            error!("game over: {}",err);
                        };
                    }
                );
            }
        }
    });
    tx
}

pub async fn start_game<N: Network, A: snarkvm::circuit::Aleo<Network = N>>(
    mut player1: Player<N>,
    mut player2: Player<N>,
    executor: Executor<N, ConsensusMemory<N>>,
) -> Result<()> {
    info!("Start game {} {}", player1.address(), player2.address());
    let mut ct = ChessTable::new(*player1.address(), *player2.address());
    player1.notify_start("A", player2.address()).await?;
    player2.notify_start("B", player1.address()).await?;
    loop {
        let requests = tokio::select! {
            req = player1.recv_request() => {
                req?
            },
            req = player2.recv_request() => {
                req?
            }
        };
        let response = executor.evaluate::<A>(requests.clone())?;
        info!("Response: {:?}", response);
        let action = action_from_request(requests[0].clone())?;
        let status = ct.update_action(action)?;
        player1
            .notify_status(status.clone(), response.clone())
            .await?;
        player2.notify_status(status, response).await?;
    }
}
