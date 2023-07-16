use crate::common::result::Result;
use crate::config::ServerConfig;

mod routes;

pub async fn launch_server(config: ServerConfig) -> Result<()> {
    let figment = rocket::Config::figment()
        .merge(("port", config.server_port));

    let _rocket = rocket::custom(figment)
        .mount("/api", routes::get_routes())
        .manage(config)
        .launch()
        .await?;

    Ok(())
}
