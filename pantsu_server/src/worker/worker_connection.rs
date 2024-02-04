use std::future::Future;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use futures::StreamExt;
use tokio_stream::wrappers::ReceiverStream;

use crate::common::{error::{self}, result::Result};

pub struct WorkerConnectionTx<J> {
    request_tx: Sender<J>,
}

impl <J> WorkerConnectionTx<J> {
    pub async fn send_job(&self, job: J) -> Result<()> {
        self.request_tx.send(job).await?;
        return Ok(());
    }
}

pub struct WorkerConnectionRx<J> {
    request_rx: Receiver<J>,
}

impl <J> WorkerConnectionRx<J> {
    pub async fn recv_job<F, Fut>(&mut self, job_handler: F) -> Result<()>
    where
        F: FnOnce(J) -> Fut,
        Fut: Future<Output = Result<()>>,
    {
        let job = self.request_rx.recv().await.ok_or_else(error::channel_receive_error)?;
        let handler_result = job_handler(job).await;
        if let Err(e) = handler_result {
            println!("Warning: handler failed: {:?}", e) // todo: log
        };
        Ok(())
    }

    pub async fn recv_stream<F, Fut>(self, job_handler: F, num_workers: usize) -> Result<()>
    where
        F: Fn(J) -> Fut,
        Fut: Future<Output = Result<()>>,
    {
        ReceiverStream::new(self.request_rx)
            .map(|job| async {
                let handler_result = job_handler(job).await;
                if let Err(e) = handler_result {
                    println!("Warning: handler failed: {:?}", e) // todo: log
                };
            })
            .buffered(num_workers)
            .for_each(|_| async {()}).await;
        Err(error::channel_receive_error())
    }
}



pub fn create_worker_connection<J>(channel_size: usize) -> (WorkerConnectionTx<J>, WorkerConnectionRx<J>) {
    let (tx, rx) = channel(channel_size);
    let connection_tx = WorkerConnectionTx {
        request_tx: tx,
    };
    let connection_rx = WorkerConnectionRx {
        request_rx: rx,
    };

    (connection_tx, connection_rx)
}
