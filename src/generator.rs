
use std::{sync::mpsc::{Sender, self}, time::Duration, thread::sleep};

use snarkvm::{synthesizer::{Authorization, Transaction, ConsensusStorage}, prelude::Network};
use tokio::sync::oneshot;

use crate::executor::Executor;
pub type ProofRequest<N> = (Authorization<N>, oneshot::Sender<Transaction<N>>);

pub fn start_generator<N: Network, C: ConsensusStorage<N>>(executor: Executor<N,C>) -> Sender<ProofRequest<N>> {
    let (tx, rx) = mpsc::channel::<ProofRequest<N>>();
    std::thread::spawn(move|| {
        while let Ok((authorization, notify)) = rx.recv() {
            match executor.execute_no_fee(authorization) {
                Ok(transaction) => {
                    if let Err(err) = notify.send(transaction) {
                        println!("Notify channel broken: {}", err);
                    }
                },
                Err(err) => {
                    println!("Execute error: {}", err);
                }
            }
            sleep(Duration::from_secs(30));
        }
    });
    tx
}