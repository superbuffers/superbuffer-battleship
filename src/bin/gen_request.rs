
use std::time::Instant;

use ::rand::thread_rng;
use clap::Parser;
use snarkvm::{prelude::*, synthesizer::helpers::memory::ConsensusMemory};
use anyhow::Result;
use superbuffer::utils::get_deployment;

#[derive(Debug, Parser)]
pub struct CLI {
    #[clap(value_parser=PrivateKey::<Testnet3>::from_str,long)]
    pub private_key: PrivateKey<Testnet3>,

    #[clap(parse(try_from_str))]
    program_id: ProgramID<Testnet3>,

    #[clap(parse(try_from_str))]
    function: Identifier<Testnet3>,
    /// The function inputs.
    #[clap(parse(try_from_str))]
    inputs: Vec<Value<Testnet3>>,
    #[clap(short, long, default_value="http://127.0.0.1:3030")]
    query: String,
}

impl CLI {
    pub fn execute(self) -> Result<()> {
        let rng = &mut thread_rng();
        let store = ConsensusStore::<Testnet3, ConsensusMemory<Testnet3>>::open(None)?;
        let vm = VM::from(store)?;
        let program: Program<Testnet3> =
        ureq::get(&format!("{}/testnet3/program/{}", self.query, self.program_id)).call()?.into_json()?;
        for (dep_program, _) in program.imports() {
            let deployment = get_deployment(&self.query, dep_program)?.unwrap();
            vm.process().write().load_deployment(&deployment)?;
        }
        println!("deploy: {}", program.id());
        let deployment = get_deployment(&self.query, program.id())?.unwrap();
        vm.process().write().load_deployment(&deployment)?;
        let start = Instant::now();
        let authorization = vm.authorize(&self.private_key, self.program_id, self.function, self.inputs.iter(), rng)?;
        let requests = authorization.to_vec_deque();

        println!("{}", serde_json::to_string(&requests)?);
        println!("elapsed: {}", start.elapsed().as_secs_f64());
        Ok(())
    }
}
fn main() {
    let cli = CLI::parse();
    cli.execute().unwrap();
}
