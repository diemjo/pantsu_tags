use std::iter::Map;
use rocket::http::Status;
use rocket::Response;
use rocket::response::content::RawJson;
use rocket::response::Responder;
use rocket::serde::json::json;
use rocket::serde::Serialize;
use rocket::tokio::sync::{mpsc, oneshot};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("reqwest client error: {0}")]
    ExampleError(String, Map<String, String>),

    // config
    #[error("error parsing config: {0}")]
    FigmentError(#[source] figment::Error),

    // rocket
    #[error("rocket error: {0}")]
    RocketError(#[from] rocket::Error),

    // channel
    #[error("channel communication error: {0}")]
    WorkerCommunicationError(String),

    #[error("received an unexpected Result: {0}, expected: {1}")]
    UnexpectedResultError(String, String)
}

impl <T> From<mpsc::error::SendError<T>> for Error {
    fn from(value: mpsc::error::SendError<T>) -> Self {
        Self::WorkerCommunicationError(format!("send failed: {}", value))
    }
}

impl From<oneshot::error::RecvError> for Error {
    fn from(value: oneshot::error::RecvError) -> Self {
        Self::WorkerCommunicationError(format!("receive failed: {}", value))
    }
}

pub fn channel_receive_error() -> Error {
    return Error::WorkerCommunicationError("receive failed: channel closed".to_string());
}

impl<'r, 'o: 'r> Responder<'r, 'o> for Error {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'o> {
        match self {
            Self::ExampleError(_, _) => self.response_with_status(request, Status::NotFound),
            _ => self.response_with_status(request, Status::InternalServerError)
        }
    }
}

impl Error {
    fn response_with_status(&self, request: &rocket::Request, status: Status) -> rocket::response::Result<'static> {
        Response::build_from(wrap_err(self.to_string()).respond_to(request).unwrap())
            .status(status)
            .ok()
    }
}

fn wrap_err<S: Serialize>(serializable: S) -> RawJson<String> {
    RawJson(serde_json::to_string(&json!({
        "messages": [json!({
                "type": "error",
                "value": serializable,
            })],
        "data": None as Option<String>,
    })).unwrap())
}
