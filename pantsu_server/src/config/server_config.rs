use std::path::PathBuf;

use figment::Figment;
use figment::providers::{Env, Format, Serialized, Yaml};
use serde::{Deserialize, Serialize};

use crate::common::error::Error;
use crate::common::result::Result;
use crate::config::ServerConfig;

impl ServerConfig {

    pub fn load_config() -> Result<Self> {
        Figment::from(Serialized::defaults(ServerConfigTemplate::default()))
            .merge(Yaml::file("/etc/pantsu-server/config.yaml"))
            .merge(Yaml::file("/config/config.yaml"))
            .merge(Yaml::file("./config.yaml"))
            .merge(Env::prefixed("PANTSU_SERVER_"))
            .extract::<ServerConfigTemplate>()
            .or_else(|e| Err(Error::FigmentError(e)))?
            .try_into()
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct ServerConfigTemplate {
    server_port: u16,
    db_username: String,
    db_password: String,
    db_url: String,
    library_path: Option<PathBuf>,
    data_form_limit: String,
    image_file_limit: String,
}

impl Default for ServerConfigTemplate {
    fn default() -> Self {
        ServerConfigTemplate {
            server_port: 8000,
            db_username: "pantsu_maid".to_string(),
            db_password: "password".to_string(),
            db_url: "localost:4269".to_string(),
            library_path: None,
            data_form_limit: "25MiB".to_string(),
            image_file_limit: "24MiB".to_string(),
        }
    }
}

impl TryFrom<ServerConfigTemplate> for ServerConfig {
    type Error = Error;

    fn try_from(value: ServerConfigTemplate) -> Result<ServerConfig> {
        Ok(ServerConfig {
            server_port: value.server_port,
            db_username: value.db_username,
            db_password: value.db_password,
            db_url: value.db_url,
            library_path: value.library_path.ok_or_else(|| Error::MissingRequiredConfigOption("library_path".to_string()))?,
            data_form_limit: value.data_form_limit.parse().map_err(|_| Error::InvalidConfigOption("data_form_limit".to_string()))?,
            image_file_limit: value.image_file_limit.parse().map_err(|_| Error::InvalidConfigOption("image_file_limit".to_string()))?,
        })
    }
}
