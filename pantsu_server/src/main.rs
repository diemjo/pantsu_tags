use rocket::main;

use crate::common::result::Result;
use crate::config::ServerConfig;

mod common;
mod config;
mod server;

#[main]
async fn main() -> Result<()> {
    let config = ServerConfig::load_config()?;

    server::launch_server(config).await?;

    Ok(())
}
