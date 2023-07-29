use std::time::Duration;
use rocket::tokio::time::sleep;
use crate::{worker::worker_connection::WorkerConnectionRx, common::result::Result};

use super::iqdb_client::{IqdbJob, IqdbResponse};


pub async fn worker_run(mut connection_rx: WorkerConnectionRx<IqdbJob, Result<IqdbResponse>>) -> Result<()>{
    /*loop {
        connection_rx.recv_job(handle_job).await?;
    }*/
    connection_rx.recv_stream(handle_job, 4).await
}

async fn handle_job(job: IqdbJob) -> Result<IqdbResponse> {
    sleep(Duration::from_secs(1)).await;
    match job {
        IqdbJob::GetSauce(image) => {
            if image.starts_with("Megumin") {
                let number = image.chars().into_iter().skip(7).collect::<String>();
                return Ok(IqdbResponse::GetSauce("Bestgirl.moe".to_string() + number.as_str()));
            }
            else {
                return Ok(IqdbResponse::GetSauce("Whatever, move the board".to_string()));
            }
        }
    }
}
