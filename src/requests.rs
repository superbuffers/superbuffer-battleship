use snarkvm::prelude::*;

pub enum ActionRequest<N: Network> {
    Initialize(Request<N>),
    Offer(Request<N>),
    Start(Request<N>),
    Play(Request<N>),
}

// impl<N: Network> ActionRequest<N> {
    // pub fn request(&self) -> Request<N> {
    //     match self {
    //         ActionRequest::Initialize(request) => request.clone(),
    //         ActionRequest::Offer(request) => request.clone(),
    //         ActionRequest::Start(request) => request.clone(),
    //         ActionRequest::Play(request) => request.clone(),
    //     }
    // }
// }

pub fn action_from_request<N: Network>(request: Request<N>) -> Result<ActionRequest<N>> {
    ensure!(request.program_id() == &ProgramID::from_str("battleship.aleo")?);
    let function_name = request.function_name().to_string();

    let action: ActionRequest<N> = match function_name.as_str() {
        "initialize_board" => ActionRequest::Initialize(request),
        "offer_battleship" => ActionRequest::Offer(request),
        "start_battleship" => ActionRequest::Start(request),
        "play" => ActionRequest::Play(request),
        _ => unreachable!(),
    };
    Ok(action)
}
