use std::time::Duration;

use rocket::tokio::time::sleep;

use crate::common::error::Error;
use crate::common::result::Result;
use crate::worker::JobResponder;
use crate::worker::worker_connection::WorkerConnectionRx;

use super::fs_service::FsJob;

pub async fn worker_run(connection_rx: WorkerConnectionRx<FsJob>) -> Result<()>{
    /*loop {
        connection_rx.recv_job(handle_job).await?;
    }*/
    connection_rx.recv_stream(handle_job, 4).await
}

async fn handle_job(job: FsJob) -> Result<()> {
    sleep(Duration::from_secs(1)).await;
    match job {
        FsJob::StoreImage(image, responder) => {
            let answer = handle_store_image(image);
            respond(responder, answer)?;
        }
    }
    Ok(())
}

fn respond<T>(responder: JobResponder<T>, response: Result<T>) -> Result<()> {
    responder.send(response)
        .map_err(|_| Error::WorkerCommunicationError("Worker unable to send response to Service".to_string()))
}

fn handle_store_image(image: String) -> Result<String> {
    Ok("fs_worker Got store image job".to_string())
}