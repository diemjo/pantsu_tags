use rocket::form::{FromFormField, ValueField, self};

use crate::image::ImageId;


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
