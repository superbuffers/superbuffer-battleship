use snarkvm::prelude::*;

pub fn get_deployment<N: Network>(base: &str, program_id: &ProgramID<N>) -> Result<Option<Deployment<N>>>{
    let transaction_id: N::TransactionID = 
    ureq::get(&format!("{base}/testnet3/find/transactionID/deployment/{program_id}")).call()?.into_json()?;
    let transaction: Transaction<N> = 
    ureq::get(&format!("{base}/testnet3/transaction/{transaction_id}")).call()?.into_json()?;
    let deployment = transaction.deployment().and_then(|deployment| Some(deployment.clone()));
    Ok(deployment)
}

pub fn vm_load_deployment<N: Network, C: ConsensusStorage<N>>(vm: &VM<N, C>, base: &str, program_id: &ProgramID<N>) -> Result<()>{
    let program: Program<N> =
    ureq::get(&format!("{base}/testnet3/program/{program_id}")).call()?.into_json()?;
    for (dep_program, _) in program.imports() {
        let deployment = get_deployment(&base, dep_program)?.expect("deployment should exsit");
        vm.process().write().load_deployment(&deployment)?;
    }
    let deployment = get_deployment(&base, program_id)?.expect("deployment should exsit");
    vm.process().write().load_deployment(&deployment)?;
    Ok(())
}