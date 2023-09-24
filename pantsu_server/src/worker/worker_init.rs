use std::future::Future;

use rocket::tokio::task;

use crate::common::result::Result;

use super::{worker_connection::{create_worker_connection, WorkerConnectionRx, WorkerConnectionTx}, web_workers::{iqdb_worker, iqdb_service::{DefaultIqdbService, IqdbService}}};


pub fn create_worker<J, F, Fut>(worker_run: F) -> WorkerConnectionTx<J>
where
    F: FnOnce(WorkerConnectionRx<J>) -> Fut + Send + 'static,
    Fut: Future<Output = Result<()>> + Send,
    J: Send + 'static,
{
    let (connection_tx, connection_rx) = create_worker_connection::<J>(128);
    task::spawn(async move {
        let _ = worker_run(connection_rx).await;
        println!("oopsie") // todo: log
    });
    return connection_tx;
}

pub fn init_iqdb() -> Box<dyn IqdbService + Send + Sync> {
    let connection_tx = create_worker(iqdb_worker::worker_run);
    let iqdb_service = DefaultIqdbService::new(connection_tx);
    return Box::new(iqdb_service);
}
