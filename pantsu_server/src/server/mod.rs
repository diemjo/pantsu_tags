use rocket::data::{Limits, ToByteUnit};
use rocket::fairing::AdHoc;
use rocket_db_pools::Database;

use crate::common::result::Result;
use crate::{Context, db, Services};
use crate::db::PantsuDB;

mod forms;
mod routes;

pub async fn launch_server(context: Context, services: Services) -> Result<()> {
    let limits = Limits::default()
        .limit("bytes", 25.mebibytes()); // image upload as bytes

    let figment = rocket::Config::figment()
        .merge(("port", context.config.server_port))
        .merge(("limits", limits))
        .merge(("databases.pantsu_db", db::config()));

    let _rocket = rocket::custom(figment)
        .mount("/api", routes::get_routes())
        .manage(context)
        .manage(services)
        .attach(PantsuDB::init())
        .attach(AdHoc::try_on_ignite("db migrations", db::migrate))
        .launch()
        .await?;

    Ok(())
}
