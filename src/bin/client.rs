

use ::rand::thread_rng;
use clap::Parser;
use snarkvm::prelude::*;

#[derive(Debug, Parser)]
pub struct CLI<N: Network> {
    #[clap(value_parser=PrivateKey::<N>::from_str,long)]
    pub private_key: PrivateKey<N>,

    #[clap(long)]
    pub to: Address<N>,

    #[clap(long)]
    pub amount: u64,

    #[clap(long)]
    pub record: Record<N, Plaintext<N>>,

    #[clap(long)]
    pub fee_record: Record<N, Plaintext<N>>,

    #[clap(long, default_value = "10000")]
    pub fee: u64,
}

fn main() {
    // let CLI {
    //     private_key,
    //     to,
    //     amount,
    //     record,
    //     fee_record,
    //     fee,
    // } = CLI::<Testnet3>::parse();
    // let storage = ConsensusMemory::open(None).unwrap();
    // let sb_client = SBClient::new(storage, private_key).unwrap();
    // let start = Instant::now();
    // let transfer_authorization = sb_client.request_transfer(to, amount, record).unwrap();
    // let fee_authorization = sb_client.request_fee(fee_record, fee).unwrap();
    // let stop = start.elapsed().as_millis();
    // println!("time: {stop}");
    // ureq::post("http://localhost:3000/execute").send_json(&(transfer_authorization, fee_authorization)).unwrap();
}

pub struct SBClient<N: Network, C: ConsensusStorage<N>> {
    vm: VM<N, C>,
    pk: PrivateKey<N>,
}

impl<N: Network, C: ConsensusStorage<N>> SBClient<N, C> {
    pub fn new(c: C, pk: PrivateKey<N>) -> Result<SBClient<N, C>> {
        let store = ConsensusStore::from(c);
        let vm = VM::from(store)?;
        let sb_client = Self { vm, pk };
        Ok(sb_client)
    }

    pub fn request_transfer(
        &self,
        to: Address<N>,
        amount: u64,
        record: Record<N, Plaintext<N>>,
    ) -> Result<Request<N>> {
        let rng = &mut thread_rng();
        let inputs = [
            Value::Record(record),
            Value::from_str(&format!("{to}"))?,
            Value::from_str(&format!("{amount}u64"))?,
        ];
        let authorization = self
            .vm
            .authorize(&self.pk, "credits.aleo", "transfer", inputs, rng)?;
        authorization.next()
    }

    pub fn request_fee(&self, fee_record: Record<N, Plaintext<N>>, fee: u64) -> Result<Request<N>> {
        let rng = &mut thread_rng();
        let inputs = [
            Value::Record(fee_record),
            Value::from_str(&format!("{fee}u64"))?,
        ];
        let authorization = self
            .vm
            .authorize(&self.pk, "credits.aleo", "fee", inputs, rng)?;
        authorization.next()
    }
}
