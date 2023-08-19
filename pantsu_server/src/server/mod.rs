use rocket::data::{Limits, ToByteUnit};

use crate::common::result::Result;
use crate::Context;

mod forms;
mod routes;

pub async fn launch_server(context: Context) -> Result<()> {
    let limits = Limits::default()
        .limit("bytes", 25.mebibytes()); // image upload as bytes

    let figment = rocket::Config::figment()
        .merge(("port", context.config.server_port))
        .merge(("limits", limits));

    let _rocket = rocket::custom(figment)
        .mount("/api", routes::get_routes())
        .manage(context)
        .launch()
        .await?;

    Ok(())
}
