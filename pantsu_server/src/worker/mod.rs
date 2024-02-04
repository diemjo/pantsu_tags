use tokio::sync::oneshot;
use crate::common::result::Result;

pub mod iqdb;
pub mod fs;
mod worker_connection;
pub mod worker_init;


pub type JobResponder<T> = oneshot::Sender<Result<T>>;
