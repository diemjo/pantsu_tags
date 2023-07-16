use figment::Figment;
use figment::providers::{Env, Format, Serialized, Yaml};

use crate::common::error::Error;
use crate::common::result::Result;
use crate::config::ServerConfig;

impl ServerConfig {

    pub fn load_config() -> Result<Self> {
        Ok(Figment::from(Serialized::defaults(ServerConfig::default()))
            .merge(Yaml::file("/etc/pantsu-server/config.yaml"))
            .merge(Yaml::file("/config/config.yaml"))
            .merge(Yaml::file("./config.yaml"))
            .merge(Env::prefixed("PANTSU_SERVER_"))
            .extract()
            .or_else(|e| Err(Error::FigmentError(e)))?
        )
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            server_port: 8000,
            db_username: "pantsu_maid".to_string(),
            db_password: "password".to_string(),
            db_url: "localost:4269".to_string(),
        }
    }
}
