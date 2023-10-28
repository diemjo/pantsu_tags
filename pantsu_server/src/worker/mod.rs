use rocket::tokio::sync::oneshot;
use crate::common::result::Result;

pub mod iqdb;
mod worker_connection;
pub mod worker_init;


pub type JobResponder<T> = oneshot::Sender<Result<T>>;
