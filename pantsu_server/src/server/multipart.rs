use axum_typed_multipart::{BaseMultipart, TypedMultipartError};
use crate::common::error::Error;
use crate::common::error::Error::MultipartError;

pub type Multipart<T> = BaseMultipart<T, Error>;

impl From<TypedMultipartError> for Error {
    fn from(value: TypedMultipartError) -> Self {
        MultipartError(value.to_string())
    }
}
