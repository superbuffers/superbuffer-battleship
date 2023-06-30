use ::rand::thread_rng;
use snarkvm::prelude::*;
use tracing::info;

use crate::utils::vm_load_deployment;
use std::collections::VecDeque;


#[derive(Clone)]
pub struct Executor<N: Network, C: ConsensusStorage<N>> {
    vm: VM<N, C>,
    query: Query<N, C::BlockStorage>,
}

impl<N: Network, C: ConsensusStorage<N>> Executor<N, C> {
    pub fn new(c: C, query: Query<N, C::BlockStorage>) -> Result<Self> {
        let store = ConsensusStore::from(c);
        let vm = VM::from(store)?;
        match &query {
            Query::VM(_) => todo!(),
            Query::REST(url) => {
                vm_load_deployment(&vm, url, &ProgramID::from_str("battleship.aleo")?)?;
                info!("vm load deployment");
            },
        }
        Ok(Self { vm, query })
    }

    pub fn evaluate<A: snarkvm::circuit::Aleo<Network = N>>(
        &self,
        requests: VecDeque<Request<N>>,
    ) -> Result<Response<N>> {
        let authorization = Authorization::new(&requests.into_iter().collect_vec());
        self.vm.process().read().evaluate::<A>(authorization)
    }
    
    pub fn execute_no_fee(&self, authorization: Authorization<N>) -> Result<Transaction<N>>{
        let rng = &mut thread_rng();
        let (_response, execution, _metrics) = self.vm.execute_authorization_raw(authorization, Some(self.query.clone()), rng)?;
        Transaction::from_execution(execution, None)
    }

    // pub fn execute(&self, func_request: Request<N>, fee_request: Request<N>) -> Result<Transaction<N>>{
    //     let rng = &mut thread_rng();
    //     let authorization = Authorization::new(&[func_request]);
    //     let (_response, execution, _metrics) = self.vm.execute_authorization_raw(authorization, Some(self.query.clone()), rng)?;

    //     let fee_authorization = Authorization::new(&[fee_request]);
    //     let (_response, fee_execution, _metrics) = self.vm.execute_authorization_raw(fee_authorization, Some(self.query.clone()), rng)?;
    //     let fee = fee_from_execution(fee_execution)?;
    //     Transaction::from_execution(execution, Some(fee))
    // }
}

pub fn fee_from_execution<N: Network>(execution: Execution<N>) -> Result<Fee<N>> {
    ensure!(execution.len() == 1);
    let transition = execution.peek()?.clone();
    let global_state_root = execution.global_state_root();
    let inclusion_proof = execution
        .inclusion_proof().cloned();
    Ok(Fee::from(transition, global_state_root, inclusion_proof))
}

// async fn execute<N: Network, C: ConsensusStorage<N>>(
//     State(executor): State<Executor<N,C>>,
//     Json((func_request, fee_request)): Json<(Request<N>, Request<N>)>
// ) {
//     let endpoint = "https://vm.aleo.org/api/testnet3/transaction/broadcast";
//     println!("Receive request: {func_request} {fee_request}");
//     let transaction = executor.execute(func_request, fee_request).unwrap();
//     println!("Transaction generated: {transaction}");
//     ureq::post(&endpoint).send_json(&transaction).unwrap();
// }
