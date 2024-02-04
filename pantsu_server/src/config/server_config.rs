use std::path::PathBuf;

use byte_unit::{Byte, Unit};
use figment::Figment;
use figment::providers::{Env, Format, Yaml};

use crate::common::error::Error;
use crate::common::result::Result;
use crate::config::ServerConfig;

impl ServerConfig {

    pub fn load_config() -> Result<Self> {
        Figment::default()
            .merge(Yaml::file("/etc/pantsu-server/config.yaml"))
            .merge(Yaml::file("/config/config.yaml"))
            .merge(Yaml::file("./config.yaml"))
            .merge(Env::prefixed("PANTSU_SERVER_"))
            .extract::<ServerConfig>()
            .or_else(|e| Err(Error::FigmentError(e)))
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            server_port: 8000,
            db_username: "pantsu_maid".to_string(),
            db_password: "password".to_string(),
            db_url: "localhost:4269".to_string(),
            library_path: PathBuf::from("pantsu_library"),
            request_body_limit: Byte::from_u64_with_unit(25, Unit::MB).unwrap(),
        }
    }
}
