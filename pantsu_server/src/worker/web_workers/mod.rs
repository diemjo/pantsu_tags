use rocket::tokio::sync::oneshot;
use crate::common::result::Result;

pub mod iqdb_service;
pub mod iqdb_worker;

pub type JobResponder<T> = oneshot::Sender<Result<T>>;
