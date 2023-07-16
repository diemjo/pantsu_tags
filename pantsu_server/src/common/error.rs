use std::iter::Map;
use rocket::http::Status;
use rocket::Response;
use rocket::response::content::RawJson;
use rocket::response::Responder;
use rocket::serde::json::json;
use rocket::serde::Serialize;
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
