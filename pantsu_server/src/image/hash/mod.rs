use image::DynamicImage;

mod id_hash;
mod perceptual_hash;

// TODO: think about implementing hashes because of changes/inconsistencies between versions/platforms

pub fn get_id_hash(bytes: &[u8]) -> String {
    id_hash::calculate_id_hash(bytes)
}

pub fn get_perceptual_hash(image: &DynamicImage) -> String {
    perceptual_hash::calculate_perceptual_hash(image)
}
