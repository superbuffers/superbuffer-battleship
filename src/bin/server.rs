use snarkvm::circuit::AleoV0;
use superbuffer::server::run;
use clap::Parser;
use snarkvm::prelude::*;


#[derive(Debug, Parser)]
pub struct CLI {
    #[clap(long, default_value="http://127.0.0.1:3030")]
    query: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let cli = CLI::parse();

    run::<Testnet3, AleoV0>(&cli.query).await
}
