use async_trait::async_trait;
use rocket::{Data, Request, Route, State};
use rocket::data::{ByteUnit, FromData, Outcome};
use rocket::response::content;
use crate::common::error::Error;

use crate::common::result::{Result, wrap_ok};
use crate::Context;

pub struct ImageData {
    bytes: Vec<u8>,
}

#[async_trait]
impl<'r> FromData<'r> for ImageData {
    type Error = Error;

    async fn from_data(_req: &'r Request<'_>, data: Data<'r>) -> Outcome<'r, Self> {
        /* let provided_hash = match req.query_value::<String>("hash") {
            Some(Ok(hash)) => hash,
            Some(Err(e)) => {
                return Error::BadRequestError(e.to_string()).to_outcome()
            },
            None => return Error::MissingParameterError("hash".to_string()).to_outcome()
        }; */
        let bytes = match data.open(ByteUnit::Mebibyte(20)).into_bytes().await { // TODO: move to config
            Ok(bytes) => {
                if bytes.is_complete() {
                    bytes.value
                }
                else {
                    return Error::RequestTooLargeError(20*1024).to_outcome() // TODO: move to config
                }
            },
            Err(e) => return Error::BadRequestError(e.to_string()).to_outcome()
        };
        /* let image = match image::load_from_memory(&bytes) {
            Ok(image) => image,
            Err(e) => return Error::NotAnImageError().to_outcome(),
        }; */
        Outcome::Success(ImageData{ bytes })
    }
}

pub fn get_routes() -> Vec<Route> {
    return rocket::routes![
        import
    ]
}

#[rocket::post("/images/import?<hash>", data = "<image>")]
pub fn import(context: &State<Context>, hash: Option<String>, image: ImageData) -> Result<content::RawJson<String>> {
    let provided_hash = hash.filter(|s| !s.is_empty())
        .ok_or_else(|| Error::MissingParameterError("hash".to_string()))?;
    Ok(wrap_ok(format!("hehe '{}'", provided_hash)))
}
