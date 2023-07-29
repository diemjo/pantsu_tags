use std::iter::Map;

use rocket::data::Outcome;
use rocket::http::Status;
use rocket::Response;
use rocket::outcome::IntoOutcome;
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

    #[error("bad request: {0}")]
    BadRequestError(String),

    #[error("file to import is not an image")]
    NotAnImageError(),

    #[error("request data is too large, max allowed size is {0}KB")]
    RequestTooLargeError(usize),

    #[error("missing required parameter: {0}")]
    MissingParameterError(String),

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

impl Error {
    fn response_with_status(&self, request: &rocket::Request, status: Status) -> rocket::response::Result<'static> {
        Response::build_from(wrap_err(self.to_string()).respond_to(request).unwrap())
            .status(status)
            .ok()
    }

    fn get_status(&self) -> Status {
        match self {
            Self::RequestTooLargeError(_) |
            Self::BadRequestError(_) |
            Self::MissingParameterError(_) => Status::BadRequest,
            _ => Status::InternalServerError,
        }
    }

    pub(crate) fn to_outcome<'r, T>(self) -> Outcome<'r, T, Error> {
        let status = self.get_status();
        Err(self).into_outcome(status)
    }
}

impl<'r, 'o: 'r> Responder<'r, 'o> for Error {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'o> {
        self.response_with_status(request, self.get_status())
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
