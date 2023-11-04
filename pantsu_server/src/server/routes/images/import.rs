use std::sync::Arc;

use rocket::{form, Route, State};
use rocket::form::{Form, FromForm};
use rocket::response::content;

use crate::common::result::{Result, wrap_ok};
use crate::server::forms::FormFile;
use crate::{Context, Services};
use crate::image::{image_id, PantsuImage};
use crate::image::image_id::ImageId;

#[derive(FromForm)]
pub struct ImageImport {
    pub image_file: FormFile,
    pub image_id: ImageId,
}

pub fn get_routes() -> Vec<Route> {
    return rocket::routes![
        import
    ]
}

#[rocket::post("/images/import", data = "<image_form>")]
pub async fn import(context: &State<Context>, services: &State<Services>, image_form: form::Result<'_, Form<ImageImport>>) -> Result<content::RawJson<String>> {
    import_impl(context, services, image_form?.into_inner()).await
}

async fn import_impl<'r>(context: &Context, services: &Services, image_import: ImageImport) -> Result<content::RawJson<String>> {
    let image = PantsuImage::try_from(&image_import.image_file.data[..])?;
    image_id::verify_image_id(&image_import.image_id, image.id())?;

    // TODO: import: check if file exists (in db), import to image directory, add to db

    let image_file_arc = Arc::new(image_import.image_file.data);
    services.fs_service.store_image(image.clone(), image_file_arc).await?;

    Ok(wrap_ok(format!("hehe '{}' '{}'", image_import.image_id.format_id_hash(), image.filename())))
}
