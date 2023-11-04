use std::{sync::Arc, io, path::PathBuf};

use rocket::tokio::{fs::{OpenOptions, DirBuilder}, io::AsyncWriteExt};

use crate::{config::ServerConfig, image::PantsuImage, common::error::Error, common::result::Result};


pub struct PantsuLibrary {
    library_path: PathBuf,
}

impl PantsuLibrary {
    pub async fn new(config: &ServerConfig) -> Result<Self> {
        DirBuilder::new()
            .recursive(true)
            .mode(0o770)
            .create(&config.library_path)
            .await
            .map_err(|err| Error::LibraryDirectoryError(config.library_path.clone(), err))?;

        return Ok(PantsuLibrary {
            library_path: config.library_path.clone()
        })
    }

    pub async fn store_image(&self, image: &PantsuImage, file_content: Arc<Vec<u8>>) -> Result<()> {
        let path = self.library_path.join(image.filename());
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .await
            .map_err(|err| match err.kind() {
                io::ErrorKind::AlreadyExists => Error::UnexpectedImageExists(image.id().clone()),
                _ => Error::IoError(err),
            })?;
        Ok(file.write_all(&file_content).await?)
    }

}
