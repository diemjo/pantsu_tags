use rocket::{form, Route, State};
use rocket::form::{Form, FromForm};
use rocket::response::content;

use crate::common::result::{Result, wrap_ok};
use crate::Context;
use crate::image::{image_id, PantsuImage};
use crate::image::image_id::ImageId;

#[derive(FromForm)]
pub struct ImageImport<'r> {
    pub image_file: &'r [u8],
    pub image_id: ImageId,
}

pub fn get_routes() -> Vec<Route> {
    return rocket::routes![
        import
    ]
}

#[rocket::post("/images/import", data = "<image_form>")]
pub fn import(context: &State<Context>, image_form: form::Result<Form<ImageImport>>) -> Result<content::RawJson<String>> {
    let image_form = image_form?.into_inner();
    let image = PantsuImage::try_from(image_form.image_file)?;
    image_id::verify_image_id(&image_form.image_id, image.id())?;
    // TODO: import: check if file exists (in db), import to image directory, add to db
    Ok(wrap_ok(format!("hehe '{}' '{}'", image_form.image_id.get_id_hash(), image.filename())))
}
