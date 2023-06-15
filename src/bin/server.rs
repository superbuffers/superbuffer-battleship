use snarkvm::{circuit::AleoV0, prelude::Testnet3};
use superbuffer::server::run;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    run::<Testnet3, AleoV0>().await
}
