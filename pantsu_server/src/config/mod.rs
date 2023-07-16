use rocket::serde::{Deserialize, Serialize};

mod server_config;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct ServerConfig {
    pub server_port: u16,
    pub db_username: String,
    pub db_password: String,
    pub db_url: String,
}
