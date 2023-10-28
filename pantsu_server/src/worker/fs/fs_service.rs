use std::sync::Arc;

use async_trait::async_trait;
use rocket::tokio::sync::oneshot;

use crate::common::result::Result;
use crate::image::PantsuImage;
use crate::worker::JobResponder;
use crate::worker::worker_connection::WorkerConnectionTx;

pub enum FsJob {
    StoreImage(PantsuImage, Arc<Vec<u8>>, JobResponder<String>)
}

#[async_trait]
pub trait FsService {
    async fn store_image(&self, image: PantsuImage, file_content: Arc<Vec<u8>>) -> Result<String>;
}

pub struct DefaultFsService {
    worker_connection: WorkerConnectionTx<FsJob>,
}

impl DefaultFsService {
    pub fn new(worker_connection: WorkerConnectionTx<FsJob>) -> Self {
        return DefaultFsService { worker_connection }
    }
}

#[async_trait]
impl FsService for DefaultFsService {
    async fn store_image(&self, image: PantsuImage, file_content: Arc<Vec<u8>>) -> Result<String> {
        let (sender, receiver) = oneshot::channel::<Result<String>>();
        let job = FsJob::StoreImage(image, file_content, sender);
        self.worker_connection.send_job(job).await?;
        return receiver.await?;
    }
}
