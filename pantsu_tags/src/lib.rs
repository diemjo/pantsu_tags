use std::path::{Path, PathBuf};
use crate::common::error;
use crate::db::{PantsuDB};
use crate::file_handler::import;

pub use crate::common::error::Error;
pub use crate::common::error::Result;
pub use crate::common::image_handle::ImageHandle;
pub use crate::common::image_handle::Sauce;
pub use crate::common::pantsu_tag::{PantsuTag, PantsuTagType};
pub use crate::sauce::SauceMatch;
pub use crate::sauce::get_thumbnail_link;

mod sauce;
mod common;
pub mod db;
pub mod file_handler;

pub const LIB_PATH: &str = "./test_image_lib/";

pub fn new_image_handle(pantsu_db: &mut PantsuDB, image_path: &Path, error_on_similar: bool) -> Result<ImageHandle> {
    let image_info = file_handler::hash::calculate_fileinfo(image_path)?;

    let image_name = image_info.filename;
    let image_res = image_info.file_res;
    if pantsu_db.get_image_transaction(image_name.as_str()).execute()?.is_some() {
        return Err(Error::ImageAlreadyExists(error::get_path(image_path)));
    }

    if error_on_similar {
        let similar = get_similar_images(&pantsu_db, &image_name, 10)?;
        if !similar.is_empty() {
            return Err(Error::SimilarImagesExist(PathBuf::from(image_path), similar))
        }
    }

    import::import_file(LIB_PATH, image_path, &image_name)?;
    let file_handle = ImageHandle::new(image_name, Sauce::NotChecked, image_res);
    pantsu_db.add_images_transaction().add_image(&file_handle).execute()?;
    Ok(file_handle)
}

pub fn get_image_sauces(image: &ImageHandle) -> Result<Vec<SauceMatch>> {
    let image_path = PathBuf::from(format!("./test_image_lib/{}", image.get_filename()));
    let mut sauce_matches = sauce::find_sauce(&image_path)?;
    sauce_matches.sort();
    sauce_matches.reverse();
    Ok(sauce_matches)
}

pub fn get_sauce_tags(sauce: &SauceMatch) -> Result<Vec<PantsuTag>> {
    sauce::find_tags_gelbooru(&sauce.link)
}

fn get_similar_images(pantsu_db: &PantsuDB, file_name: &str, min_dist: u32) -> Result<Vec<ImageHandle>> {
    let files = pantsu_db.get_images_transaction().execute()?;
    file_handler::hash::get_similarity_distances(file_name, files, min_dist)
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use std::path::PathBuf;
    use crate::{PantsuDB, Sauce};
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_add_image() {
        let mut db_path = std::env::current_dir().unwrap();
        db_path.push("pantsu_tags.db");
        let mut pdb = PantsuDB::new(&db_path).unwrap();
        let image_path = prepare_image("https://img1.gelbooru.com/images/4f/76/4f76b8d52983af1d28b1bf8d830d684e.png");

        let image_handle = crate::new_image_handle(&mut pdb, &image_path, false).unwrap();
        let sauces = crate::get_image_sauces(&image_handle).unwrap();
        let best_match = &sauces[0];
        // in general, you would want to check the similarity here
        let tags = crate::get_sauce_tags(&best_match).unwrap();
        pdb.update_images_transaction().for_image(image_handle.get_filename()).update_sauce(&Sauce::Match(best_match.link.clone())).add_tags(&tags).execute().unwrap();
    }

    #[test]
    #[serial]
    fn test_similar_images() {
        let image_path = prepare_image("https://img1.gelbooru.com/images/4f/76/4f76b8d52983af1d28b1bf8d830d684e.png");
        let similar_image_path = prepare_image("https://img1.gelbooru.com/images/22/3a/223a6444a6e79ecb273896cfccee9850.png");
        let not_similar_image_path = prepare_image("https://img3.gelbooru.com/images/1d/b8/1db8ab6c94e95f36a9dd5bde347f6dd1.png");
        let mut db_path = std::env::current_dir().unwrap();
        db_path.push("pantsu_tags.db");
        let mut pdb = PantsuDB::new(&db_path).unwrap();
        pdb.clear().unwrap();

        crate::new_image_handle(&mut pdb, &image_path, true).unwrap();
        crate::new_image_handle(&mut pdb, &not_similar_image_path, true).unwrap();
        crate::new_image_handle(&mut pdb, &similar_image_path, true).unwrap_err();
    }

    fn prepare_image(image_link: &str) -> PathBuf {
        let image_name = image_link.rsplit('/').next().unwrap();
        let image_path = PathBuf::from(format!("test_image_{}", image_name));
        if image_path.exists() {
            return image_path;
        }

        let response = reqwest::blocking::get(image_link).unwrap();
        let mut file = std::fs::File::create(&image_path).unwrap();
        let mut content =  Cursor::new(response.bytes().unwrap());
        std::io::copy(&mut content, &mut file).unwrap();
        image_path
    }
}