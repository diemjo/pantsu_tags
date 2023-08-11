use async_trait::async_trait;
use rocket::form::{FromForm, Form};
use rocket::fs::TempFile;
use rocket::{Data, Request, Route, State};
use rocket::data::{ByteUnit, FromData, Outcome};
use rocket::response::content;
use crate::common::error::Error;

use crate::common::result::{Result, wrap_ok};
use crate::Context;
use crate::image::ImageId;

pub struct ImageData {
    bytes: Vec<u8>,
}


#[derive(FromForm)]
pub struct ImageImport<'r> {
    pub image: TempFile<'r>,
    pub image_id: ImageId,
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
        let bytes = match data.open(ByteUnit::Mebibyte(20)).into_bytes().await { // TODO: move max filesize to config
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

#[rocket::post("/images/import", data = "<image_form>")]
pub fn import(context: &State<Context>, image_form: Form<ImageImport>) -> Result<content::RawJson<String>> {
    /*let image_form = match image_form {
        Ok(image_form) => Ok(image_form),
        Err(e) => match e[0].kind {
            ErrorKind::Custom(e) => match e.downcast::<crate::common::error::Error>() {
                Ok(e) => Err(*e),
                _ => Err(Error::BadRequestError("".to_string())),
            },
            _ => Err(Error::BadRequestError("".to_string())),
            
        }
    }?;*/
    Ok(wrap_ok(format!("hehe '{}'", image_form.image_id.get_id_hash())))
}
