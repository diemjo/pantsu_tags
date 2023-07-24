use std::future::Future;

use rocket::{tokio::sync::{oneshot, mpsc::{Sender, Receiver, channel}}, futures::StreamExt};
use tokio_stream::wrappers::ReceiverStream;

use crate::common::{result::Result, error::{Error, self}};

struct JobRequest<J, R> {
    job: J,
    response_tx: oneshot::Sender<R>,
}

pub struct WorkerConnectionTx<J, R> {
    request_tx: Sender<JobRequest<J, R>>,
}

impl <J, R> WorkerConnectionTx<J, R> {
    pub async fn send_job(&self, job: J) -> Result<R> {
        let (response_tx, response_rx) = oneshot::channel::<R>();
        let request = JobRequest {
            job,
            response_tx
        };
        self.request_tx.send(request).await?;
        let response: R = response_rx.await?;
        return Ok(response);
    }
}

impl <J, R> Clone for WorkerConnectionTx<J, R> {
    fn clone(&self) -> Self {
        Self { request_tx: self.request_tx.clone() }
    }
}

pub struct WorkerConnectionRx<J, R> {
    request_rx: Receiver<JobRequest<J, R>>,
}

impl <J, R> WorkerConnectionRx<J, R> {
    pub async fn recv_job<F, Fut>(&mut self, job_handler: F) -> Result<()>
    where
        F: FnOnce(J) -> Fut,
        Fut: Future<Output = R>,
    {
        let job_request = self.request_rx.recv().await.ok_or_else(error::channel_receive_error)?;
        let resp = job_handler(job_request.job).await;
        let _ = job_request.response_tx.send(resp).map_err(|_| println!("Warning: Response channel closed")); // todo: log
        Ok(())
    }

    pub async fn recv_stream<F, Fut>(self, job_handler: F, num_workers: usize) -> Result<()>
    where
        F: Fn(J) -> Fut,
        Fut: Future<Output = R>,
    {
        ReceiverStream::new(self.request_rx)
            .map(|job_request| async {
                let resp = job_handler(job_request.job).await;
                let _ = job_request.response_tx.send(resp).map_err(|_| println!("Warning: Response channel closed")); // todo: log
            })
            .buffered(num_workers)
            .for_each(|_| async {()}).await;
        Err(error::channel_receive_error())
    }
}



pub fn create_worker_connection<J, R>(channel_size: usize) -> (WorkerConnectionTx<J, R>, WorkerConnectionRx<J, R>) {
    let (tx, rx) = channel(channel_size);
    let connection_tx = WorkerConnectionTx {
        request_tx: tx,
    };
    let connection_rx = WorkerConnectionRx {
        request_rx: rx,
    };

    (connection_tx, connection_rx)
}
