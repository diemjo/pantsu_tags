use image::DynamicImage;

pub fn calculate_perceptual_hash(image: &DynamicImage) -> String {
    let hash = blockhash::blockhash144(image);
    hash.to_string()
}
