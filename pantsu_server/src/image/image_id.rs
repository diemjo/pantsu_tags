use std::fmt::{Display, Formatter};
use std::num::ParseIntError;

use regex::Regex;

use crate::common::error::Error;
use crate::common::result::Result;
use crate::image::hash::{IdHash, PerceptualHash};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageId {
    id_hash: IdHash,
    perceptual_hash: PerceptualHash,
}

impl ImageId {
    pub fn new(id_hash: IdHash, perceptual_hash: PerceptualHash) -> Self {
        ImageId {
            id_hash,
            perceptual_hash,
        }
    }

    pub fn get_id_hash(&self) -> &[u8] {
        &self.id_hash
    }

    pub fn get_perceptual_hash(&self) -> &[u8] {
        &self.perceptual_hash
    }

    pub fn filename_format(&self) -> String {
        format!("{}-{}", self.format_id_hash(), self.format_perceptual_hash())
    }

    pub fn format_id_hash(&self) -> String {
        hash_to_hex(&self.id_hash)
    }

    pub fn format_perceptual_hash(&self) -> String {
        hash_to_hex(&self.perceptual_hash)
    }
}

fn hash_to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x?}", b)).collect::<String>()
}

impl TryFrom<&str> for ImageId {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        let regex = Regex::new(r"^(?<id>[[:xdigit:]]{16})-(?<perceptual>[[:xdigit:]]{36})$").unwrap();
        let captures = regex.captures(value.trim())
            .ok_or_else(|| Error::InvalidImageId(value.to_string()))?;
        let id_hash: IdHash = hex_to_hash::<8>(&captures["id"])?;
        let perceptual_hash: PerceptualHash = hex_to_hash::<18>(&captures["perceptual"])?;
        Ok(ImageId {
            id_hash,
            perceptual_hash,
        })
    }
}

/// should be infallible, because we already check for valid size and content of `str` with regex
fn hex_to_hash<const SIZE: usize>(str: &str) -> Result<[u8; SIZE]> {
    (0..2*SIZE)
        .step_by(2)
        .map(|i| u8::from_str_radix(&str[i..i+2], 16))
        .collect::<std::result::Result<Vec<u8>, ParseIntError>>()
        .map_err(|_| Error::InvalidImageId(str.to_owned()))?
        .try_into()
        .map_err(|_| Error::InvalidImageId(str.to_owned()))
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
