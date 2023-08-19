use std::fmt::{Display, Formatter, Write};

use regex::Regex;

use crate::common::error::Error;
use crate::common::result::Result;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageId {
    id_hash: String,
    perceptual_hash: String,
}

impl ImageId {
    pub fn new(id_hash: String, perceptual_hash: String) -> Self {
        ImageId {
            id_hash,
            perceptual_hash,
        }
    }

    pub fn get_id_hash(&self) -> &str {
        &self.id_hash
    }

    pub fn get_perceptual_hash(&self) -> &str {
        &self.perceptual_hash
    }

    pub fn filename_format(&self) -> String {
        format!("{}-{}", self.id_hash, self.perceptual_hash)
    }
}

impl TryFrom<&str> for ImageId {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        let regex = Regex::new(r"^(?<id>[[:xdigit:]]{16})-(?<perceptual>[[:xdigit:]]{36})$").unwrap();
        let captures = regex.captures(value.trim())
            .ok_or_else(|| Error::InvalidImageId(value.to_string()))?;

        Ok(ImageId {
            id_hash: captures["id"].to_string(),
            perceptual_hash: captures["perceptual"].to_string(),
        })
    }
}

impl Display for ImageId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.filename_format())
    }
}

pub fn verify_image_id(provided: &ImageId, expected: &ImageId) -> Result<()> {
    if provided != expected {
        return Err(Error::ImageIdDoesNotMatch(provided.clone(), expected.clone()))
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::common::error::Error;

    use super::ImageId;

    #[test]
    fn creates_image_id_from_correct_string() {
        let name = "a8c65b2726296dcc-07807e4fe23cb3c1dca0ce71f382bf81f00f";
        ImageId::try_from(name).unwrap();
    }

    #[test]
    fn empty_string_is_invalid() {
        let name = "";
        let image_id = ImageId::try_from(name).unwrap_err();
        assert!(matches!(image_id, Error::InvalidImageId(_)));
    }

    #[test]
    fn no_dash_is_invalid() {
        let name = "a8c65b2726296dcc07807e4fe23cb3c1dca0ce71f382bf81f00f";
        let image_id = ImageId::try_from(name).unwrap_err();
        assert!(matches!(image_id, Error::InvalidImageId(_)));
    }

    #[test]
    fn non_hex_string_is_invalid() {
        let name = "a8c65j2726296dcc-07807e4fe23cb3c1dca0ce71f382bf81f00f";
        let image_id = ImageId::try_from(name).unwrap_err();
        assert!(matches!(image_id, Error::InvalidImageId(_)));
    }

    #[test]
    fn too_short_id_hash_is_invalid() {
        let name = "a8c652726296dcc-07807e4fe23cb3c1dca0ce71f382bf81f00f";
        let image_id = ImageId::try_from(name).unwrap_err();
        assert!(matches!(image_id, Error::InvalidImageId(_)));
    }

    #[test]
    fn too_short_perceptual_hash_is_invalid() {
        let name = "a8c65b2726296dcc-07807e4fe23cb3c1dca0ce71f382bf8100f";
        let image_id = ImageId::try_from(name).unwrap_err();
        assert!(matches!(image_id, Error::InvalidImageId(_)));
    }

    #[test]
    fn excess_string_is_invalid() {
        let name = "a8c65b2726296dcc-07807e4fe23cb3c1dca0ce71f382bf81f00f HelloThere";
        let image_id = ImageId::try_from(name).unwrap_err();
        assert!(matches!(image_id, Error::InvalidImageId(_)));
    }
}
