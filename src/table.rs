
use anyhow::{ensure, Result};
use snarkvm::prelude::*;

use crate::requests::ActionRequest;

#[derive(Debug, Clone,Serialize, Deserialize)]
pub enum Status {
    AInitialize,
    BInitialize,
    AOffer,
    BStart,
    ATurn,
    BTurn,
}

// ChessTable is a statemachine
pub struct ChessTable<N: Network> {
    player_a: Address<N>,
    player_b: Address<N>,
    requests: Vec<Request<N>>,
    // responses: Vec<Response<N>>,
    status: Status,
}

impl<N: Network> ChessTable<N> {
    pub fn new(player_a: Address<N>, player_b: Address<N>) -> ChessTable<N> {
        Self {
            player_a,
            player_b,
            requests: vec![],
            // responses: vec![],
            status: Status::AInitialize,
        }
    }

    pub fn update_action(&mut self, action: ActionRequest<N>) -> Result<Status> {
        match action {
            ActionRequest::Initialize(request) => {
                self.update_initial_request(request)?;
            }
            ActionRequest::Offer(request) => {
                self.update_offer_request(request)?;
            }
            ActionRequest::Start(request) => {
                self.update_start_request(request)?;
            }
            ActionRequest::Play(request) => {
                self.update_play_request(request)?;
            }
        }
        Ok(self.status.clone())
    }

    fn update_initial_request(&mut self, request: Request<N>) -> Result<()> {
        match self.status {
            Status::AInitialize => {
                ensure!(request.caller() == &self.player_a);
                self.requests.push(request);
                self.status = Status::BInitialize;
            }
            Status::BInitialize => {
                ensure!(request.caller() == &self.player_b);
                self.requests.push(request);
                self.status = Status::AOffer;
            }
            _ => bail!(
                "Game state: {:?}, recv request: {:?}",
                self.status,
                request.function_name()
            ),
        }
        Ok(())
    }

    fn update_offer_request(&mut self, request: Request<N>) -> Result<()> {
        match self.status {
            Status::AOffer => {
                ensure!(request.caller() == &self.player_a);
                self.requests.push(request);
                self.status = Status::BStart;
            }
            _ => bail!(
                "Game state: {:?}, recv request: {:?}",
                self.status,
                request.function_name()
            ),
        }
        Ok(())
    }

    fn update_start_request(&mut self, request: Request<N>) -> Result<()> {
        match self.status {
            Status::BStart => {
                ensure!(request.caller() == &self.player_b);
                self.requests.push(request);
                self.status = Status::ATurn;
            }
            _ => bail!(
                "Game state: {:?}, recv request: {:?}",
                self.status,
                request.function_name()
            ),
        }
        Ok(())
    }

    fn update_play_request(&mut self, request: Request<N>) -> Result<()> {
        match self.status {
            Status::ATurn => {
                ensure!(request.caller() == &self.player_a);
                self.requests.push(request);
                self.status = Status::BTurn;
            }
            Status::BTurn => {
                ensure!(request.caller() == &self.player_b);
                self.requests.push(request);
                self.status = Status::ATurn;
            }
            _ => bail!(
                "Game state: {:?}, recv request: {:?}",
                self.status,
                request.function_name()
            ),
        }
        Ok(())
    }
}
