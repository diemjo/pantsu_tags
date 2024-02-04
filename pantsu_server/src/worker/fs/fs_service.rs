use async_trait::async_trait;
use bytes::Bytes;
use tokio::sync::oneshot;

use crate::common::result::Result;
use crate::image::PantsuImage;
use crate::worker::JobResponder;
use crate::worker::worker_connection::WorkerConnectionTx;

pub enum FsJob {
    StoreImage(PantsuImage, Bytes, JobResponder<()>)
}

#[async_trait]
pub trait FsService {
    async fn store_image(&self, image: PantsuImage, file_content: Bytes) -> Result<()>;
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
    async fn store_image(&self, image: PantsuImage, file_content: Bytes) -> Result<()> {
        let (sender, receiver) = oneshot::channel::<Result<()>>();
        let job = FsJob::StoreImage(image, file_content, sender);
        self.worker_connection.send_job(job).await?;
        return receiver.await?;
    }
}
