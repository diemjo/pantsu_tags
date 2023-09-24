use std::time::Duration;
use rocket::tokio::time::sleep;
use crate::{worker::worker_connection::WorkerConnectionRx, common::result::Result};
use crate::common::error::Error;
use crate::worker::web_workers::JobResponder;

use super::iqdb_service::{IqdbJob};


pub async fn worker_run(connection_rx: WorkerConnectionRx<IqdbJob>) -> Result<()>{
    /*loop {
        connection_rx.recv_job(handle_job).await?;
    }*/
    connection_rx.recv_stream(handle_job, 4).await
}

async fn handle_job(job: IqdbJob) -> Result<()> {
    sleep(Duration::from_secs(1)).await;
    match job {
        IqdbJob::GetSauce(image, responder) => {
            let answer = handle_get_sauce(image);
            respond(responder, answer)?;
        }
    }
    Ok(())
}

fn respond<T>(responder: JobResponder<T>, response: Result<T>) -> Result<()> {
    responder.send(response)
        .map_err(|_| Error::WorkerCommunicationError("connection not working, oopsie".to_string()))
}

fn handle_get_sauce(image: String) -> Result<String> {
    if image.starts_with("Megumin") {
        let number = image.chars().into_iter().skip(7).collect::<String>();
        Ok("Bestgirl.moe".to_string() + number.as_str())
    }
    else {
        Ok("Whatever, move the board".to_string())
    }
}
