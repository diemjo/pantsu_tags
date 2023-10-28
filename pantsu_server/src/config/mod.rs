use std::path::PathBuf;

use rocket::{serde::{Deserialize, Serialize}, data::ByteUnit};

mod server_config;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct ServerConfig {
    pub server_port: u16,
    pub db_username: String,
    pub db_password: String,
    pub db_url: String,
    pub library_path: PathBuf,
    pub data_form_limit: ByteUnit,
    pub image_file_limit: ByteUnit,
}
