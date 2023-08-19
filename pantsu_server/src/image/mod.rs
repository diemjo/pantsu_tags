use chrono::{DateTime, Utc};
use image::GenericImageView;

use crate::common::error::Error;
use crate::common::result::Result;
use crate::image::image_id::ImageId;

pub mod hash;
pub mod image_id;

enum ImageFormat {
    PNG, JPG,
}

impl ImageFormat {
    fn extension(&self) -> String {
        match self {
            ImageFormat::PNG => "png".to_string(),
            ImageFormat::JPG => "jpg".to_string(),
        }
    }
}

impl TryFrom<image::ImageFormat> for ImageFormat {
    type Error = Error;
    fn try_from(format: image::ImageFormat) -> Result<Self> {
        Ok(match format {
            image::ImageFormat::Png => ImageFormat::PNG,
            image::ImageFormat::Jpeg => ImageFormat::JPG,
            f => return Err(Error::UnsupportedImageFormat(f.to_mime_type().to_string()))
        })
    }
}

pub struct PantsuImage {
    id: ImageId,
    format: ImageFormat,
    dimensions: (u32, u32),
    date_added: DateTime<Utc>,
}

impl PantsuImage {
    pub fn id(&self) -> &ImageId {
        &self.id
    }

    pub fn filename(&self) -> String {
        format!("{}-{}.{}", self.id.get_id_hash(), self.id.get_perceptual_hash(), self.format.extension())
    }
}

impl TryFrom<&[u8]> for PantsuImage {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Self> {
        let image = image::load_from_memory(bytes)?;
        let image_id_hash = hash::get_id_hash(bytes);
        let image_perceptual_hash = hash::get_perceptual_hash(&image);
        let image_dimensions = image.dimensions();
        let image_format = ImageFormat::try_from(image::guess_format(bytes)?)?;
        Ok(
            PantsuImage {
                id: ImageId::new(image_id_hash, image_perceptual_hash),
                format: image_format,
                dimensions: image_dimensions,
                date_added: Utc::now(),
            }
        )
    }
}
