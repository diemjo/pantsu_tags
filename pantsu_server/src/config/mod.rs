use std::path::PathBuf;
use std::str::FromStr;
use byte_unit::Byte;
use serde::{Deserialize};
use serde::de::Error;

mod server_config;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(default)]
pub struct ServerConfig {
    pub server_port: u16,
    pub db_username: String,
    pub db_password: String,
    pub db_url: String,
    pub library_path: PathBuf,
    #[serde(deserialize_with = "parse_byte")]
    pub request_body_limit: Byte,
}

fn parse_byte<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<Byte, D::Error> {
    let value: String = String::deserialize(deserializer)?;
    Byte::from_str(value.as_str()).map_err(Error::custom)
}
