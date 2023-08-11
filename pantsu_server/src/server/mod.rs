use crate::Context;
use crate::common::result::Result;

mod forms;
mod routes;

pub async fn launch_server(context: Context) -> Result<()> {
    let figment = rocket::Config::figment()
        .merge(("port", context.config.server_port));

    let _rocket = rocket::custom(figment)
        .mount("/api", routes::get_routes())
        .manage(context)
        .launch()
        .await?;

    Ok(())
}
