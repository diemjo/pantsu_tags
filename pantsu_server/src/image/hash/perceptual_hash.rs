use image::DynamicImage;

pub fn calculate_perceptual_hash(image: &DynamicImage) -> [u8; 18] {
    blockhash::blockhash144(image).into()
}
