use async_trait::async_trait;
use tokio::sync::oneshot;

use crate::{common::result::Result, worker::worker_connection::WorkerConnectionTx};
use crate::worker::JobResponder;

pub enum IqdbJob {
    GetSauce(String, JobResponder<String>)
}

#[async_trait]
pub trait IqdbService {
    async fn get_sauce(&self, image: String) -> Result<String>;
}

pub struct DefaultIqdbService {
    worker_connection: WorkerConnectionTx<IqdbJob>,
}

impl DefaultIqdbService {
    pub fn new(worker_connection: WorkerConnectionTx<IqdbJob>) -> Self {
        return DefaultIqdbService { worker_connection }
    }
}

#[async_trait]
impl IqdbService for DefaultIqdbService {
    async fn get_sauce(&self, image: String) -> Result<String> {
        let (sender, receiver) = oneshot::channel::<Result<String>>();
        let job = IqdbJob::GetSauce(image, sender);
        self.worker_connection.send_job(job).await?;
        return receiver.await?;
    }
}
