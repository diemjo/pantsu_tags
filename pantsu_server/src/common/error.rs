use std::iter::Map;

use image::ImageError;
use rocket::{form, Response};
use rocket::data::Outcome;
use rocket::form::error::ErrorKind;
use rocket::http::Status;
use rocket::outcome::IntoOutcome;
use rocket::response::content::RawJson;
use rocket::response::Responder;
use rocket::serde::json::json;
use rocket::serde::Serialize;
use rocket::tokio::sync::{mpsc, oneshot};
use rocket_db_pools::{deadpool_postgres};
use thiserror::Error;

use crate::common::option_ext::OptionExt;
use crate::image::image_id::ImageId;

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
    RequestTooLargeError(u64),

    #[error("missing required parameter: {0}")]
    MissingParameterError(String),

    // channel
    #[error("channel communication error: {0}")]
    WorkerCommunicationError(String),

    #[error("received an unexpected Result: {0}, expected: {1}")]
    UnexpectedResultError(String, String),

    // image
    #[error("Provided image id is invalid: {0}")]
    InvalidImageId(String),

    #[error("Provided image id '{0}' is not equal to the actual image id: '{1}'")]
    ImageIdDoesNotMatch(ImageId, ImageId),

    #[error("Provided image is invalid: {0}")]
    InvalidImageFile(#[from] ImageError),

    #[error("Provided image format is unsupported: {0}")]
    UnsupportedImageFormat(String),

    // database
    #[error("Database sql error: {0}")]
    DbSqlError(#[from] deadpool_postgres::tokio_postgres::Error),

    #[error("Database pool error: {0}")]
    DbPoolError(#[from] deadpool_postgres::PoolError),

    #[error("Database pool error: {0}")]
    DbMigrationError(#[source] deadpool_postgres::tokio_postgres::Error),

    #[error("Program is outdated, database is on version {0}, expected <={1}")]
    ProgramOutdatedError(usize, usize),
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

impl<'r> From<form::Errors<'r>> for Error {
    fn from(value: form::Errors) -> Self {
        let error = match value.into_iter().next() {
            Some(error) => error,
            None => return Error::BadRequestError("Error without error kind encountered".to_string()),
        };
        match error.kind {
            ErrorKind::Custom(box_error) => match box_error.downcast::<Error>() {
                Ok(e) => *e,
                Err(e) => return Error::BadRequestError(e.to_string()),
            }
            ErrorKind::Missing => return Error::MissingParameterError(error.name.unwrap_or_unknown()),
            ErrorKind::InvalidLength { min: _, max} => return Error::RequestTooLargeError(max.map(|m| m/1024).unwrap_or(0)),
            _ => return Error::BadRequestError(format!("Unknown error kind: {}", error.kind)),
        }
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
            Self::MissingParameterError(_) |
            Self::InvalidImageId(_) => Status::BadRequest,
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
