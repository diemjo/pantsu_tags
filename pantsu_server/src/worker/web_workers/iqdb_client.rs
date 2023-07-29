use async_trait::async_trait;

use crate::{common::{result::Result, error::Error}, worker::worker_connection::WorkerConnectionTx};


pub enum IqdbJob {
    GetSauce(String)
}

pub enum IqdbResponse {
    GetSauce(String)
}

#[async_trait]
pub trait IqdbClient {
    async fn get_sauce(&self, image: String) -> Result<String>;
}

pub struct DefaultIqdbClient {
    worker_connection: WorkerConnectionTx<IqdbJob, Result<IqdbResponse>>,
}

impl DefaultIqdbClient {
    pub fn new(worker_connection: WorkerConnectionTx<IqdbJob, Result<IqdbResponse>>) -> Self {
        return DefaultIqdbClient { worker_connection }
    }
}

#[async_trait]
impl IqdbClient for DefaultIqdbClient {
    async fn get_sauce(&self, image: String) -> Result<String> {
        let job = IqdbJob::GetSauce(image);
        return match self.worker_connection.send_job(job).await?? {
            IqdbResponse::GetSauce(sauce) => Ok(sauce),
            _ => Err(Error::UnexpectedResultError("wrong".to_string(), "getSauce".to_string()))
        };
    }
}
