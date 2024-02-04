use std::iter::Map;
use std::path::PathBuf;

use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use image::ImageError;
use thiserror::Error;
use tokio::sync::{mpsc, oneshot};
use tracing::dispatcher::SetGlobalDefaultError;

use crate::image::image_id::ImageId;

#[derive(Error, Debug)]
pub enum Error {
    #[error("reqwest client error: {0}")]
    ExampleError(String, Map<String, String>),

    // config
    #[error("error parsing config: {0}")]
    FigmentError(#[source] figment::Error),

    // log
    #[error("error setting global logger")]
    LogInitError(#[from] SetGlobalDefaultError),

    // axum
    #[error("bad request: {0}")]
    BadRequestError(String),

    #[error("file to import is not an image")]
    NotAnImageError(),

    #[error("request data is too large, max allowed size is {0}KB")]
    RequestTooLargeError(u64),

    #[error("multipart error: {0}")]
    MultipartError(String),

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

    // filesystem
    #[error("Library directory '{0}' does not exist and cannot be created due to error: {1}")]
    LibraryDirectoryError(PathBuf, std::io::Error),

    #[error("Image exists on disk, but should not according to database: {0}")]
    UnexpectedImageExists(ImageId),

    #[error("Encountered an unexpected IO Error: '{0}'")]
    IoError(#[from] std::io::Error),

    // database
//     #[error("Database sql error: {0}")]
//     DbSqlError(#[from] deadpool_postgres::tokio_postgres::Error),
//
//     #[error("Database pool error: {0}")]
//     DbPoolError(#[from] deadpool_postgres::PoolError),
//
//     #[error("Migration error: {0}")]
//     DbMigrationError(#[source] deadpool_postgres::tokio_postgres::Error),
//
//     #[error("Program is outdated, database is on version {0}, expected <={1}")]
//     ProgramOutdatedError(String, String),
//
//     #[error("migrations are missing version {0}")]
//     DbMigrationVersionMissing(String),
//
//     #[error("migration hashes do not match for version {0}: applied '{1}', expected '{2}'")]
//     DbMigrationHashMismatch(String, String, String),
}

impl Error {

    fn get_status_code(&self) -> StatusCode {
        match self {
            Self::RequestTooLargeError(_) |
            Self::BadRequestError(_) |
            Self::MissingParameterError(_) |
            Self::InvalidImageId(_) |
            Self::MultipartError(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (self.get_status_code(), Json(self.to_string())).into_response()
    }
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
