use std::sync::Arc;

use axum::extract::State;
use axum_typed_multipart::TryFromMultipart;
use bytes::Bytes;
use tracing::log::{debug, info};

use crate::AppState;
use crate::common::result;
use crate::image::{image_id, PantsuImage};
use crate::image::image_id::ImageId;
use crate::server::multipart::Multipart;
use crate::worker::fs::fs_service::FsService;

#[derive(TryFromMultipart, Debug)]
pub struct ImageImport {
    #[form_data(limit = "unlimited")]
    pub image_file: Bytes,
    pub image_id: ImageId,
}
pub async fn import(State(state): State<AppState>, image_import: Multipart<ImageImport>) -> result::Result<()> {
    debug!("{:?}", image_import.image_id);
    import_impl(state.fs_service.clone(), image_import.data).await?;
    Ok(())
}

async fn import_impl(fs_service: Arc<dyn FsService + Sync + Send>, image_import: ImageImport) -> result::Result<()> {
    let image = PantsuImage::try_from(image_import.image_file.as_ref())?;
    image_id::verify_image_id(&image_import.image_id, image.id())?;

    info!("Store image in library: '{}'", image.filename());
    fs_service.store_image(image.clone(), image_import.image_file.clone()).await?;
    Ok(())
}

// async fn import_impl2<'r>(context: &Context, services: &Services, image_import: ImageImport) -> Result<content::RawJson<String>> {
//     let image = PantsuImage::try_from(&image_import.image_file.data[..])?;
//     image_id::verify_image_id(&image_import.image_id, image.id())?;
//
//     // TODO: import: check if file exists (in db)
//
//     info!("Store image in library: '{}'", image.filename());
//     let image_file_arc = Arc::new(image_import.image_file.data);
//     services.fs_service.store_image(image.clone(), image_file_arc.clone()).await?;
//
//     // TODO: add to db
//     Ok(wrap_ok(format!("hehe '{}' '{}'", image_import.image_id.format_id_hash(), image.filename())))
// }
