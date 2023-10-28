use rocket::{form::{FromFormField, ValueField, self, DataField}, http::ContentType};

use crate::{image::image_id::ImageId, common::error::Error};


#[rocket::async_trait]
impl <'r> FromFormField<'r> for ImageId {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        match ImageId::try_from(field.value) {
            Ok(image_id) => Ok(image_id),
            //Err(e) => Err(form::Error::validation(e.to_string()))?
            Err(e) => Err(form::Error::custom(e))?
        }
    }
}

pub struct FormFile {
    pub content_type: ContentType,
    pub data: Vec<u8>,
}

#[rocket::async_trait]
impl<'v> FromFormField<'v> for FormFile {
    async fn from_data(field: DataField<'v, '_>) -> form::Result<'v, Self> {
        let size_limit = field.request.limits().get("image-file")
            .ok_or_else(|| form::error::Error::custom(Error::NoLimitConfiguredError("image-file".to_string())))?;
        let stream = field.data.open(size_limit);
        let bytes = stream.into_bytes().await?;
        if !bytes.is_complete() {
            Err((None, Some(size_limit)))?;
        }

        Ok(FormFile { 
            content_type: field.content_type,
            data: bytes.value,
        })
    }
}