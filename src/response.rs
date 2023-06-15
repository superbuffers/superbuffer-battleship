use anyhow::Result;
use snarkvm::prelude::*;

pub trait ActionResponse {
    fn dispatch(&self) -> Result<()>;
}

pub struct InitializeResponse<N: Network>(Response<N>);
pub struct OfferReponse<N: Network>(Response<N>);
pub struct StartReponse<N: Network>(Response<N>);
pub struct PlayResponse<N: Network>(Response<N>);

impl<N: Network> ActionResponse for InitializeResponse<N> {
    fn dispatch(&self) -> Result<()> {
        ensure!(self.0.outputs().len() == 1);
        let board_record = self.0.outputs()[0].clone();
        matches!(board_record, Value::Record(..));
        Ok(())
    }
}

impl<N: Network> ActionResponse for OfferReponse<N> {
    fn dispatch(&self) -> Result<()> {
        ensure!(self.0.outputs().len() == 2);
        let board_record = self.0.outputs()[0].clone();
        let move_record = self.0.outputs()[1].clone();
        matches!(board_record, Value::Record(..));
        matches!(move_record, Value::Record(..));
        Ok(())
    }
}

impl<N: Network> ActionResponse for StartReponse<N> {
    fn dispatch(&self) -> Result<()> {
        ensure!(self.0.outputs().len() == 2);
        let board_record = self.0.outputs()[0].clone();
        let move_record = self.0.outputs()[1].clone();
        matches!(board_record, Value::Record(..));
        matches!(move_record, Value::Record(..));
        Ok(())
    }
}

impl<N: Network> ActionResponse for PlayResponse<N> {
    fn dispatch(&self) -> Result<()> {
        ensure!(self.0.outputs().len() == 2);
        let board_record = self.0.outputs()[0].clone();
        let move_record = self.0.outputs()[1].clone();
        matches!(board_record, Value::Record(..));
        matches!(move_record, Value::Record(..));
        Ok(())
    }
}
