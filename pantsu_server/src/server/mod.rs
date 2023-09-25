use rocket::data::{Limits, ToByteUnit};
use rocket::fairing::AdHoc;
use rocket_db_pools::Database;

use crate::common::result::Result;
use crate::Context;
use crate::db;
use crate::db::PantsuDB;

mod forms;
mod routes;

pub async fn launch_server(context: Context) -> Result<()> {
    let db_config = rocket_db_pools::Config {
        url: "postgres://postgres:postgres@localhost/pantsudb".to_string(),
        min_connections: None,
        max_connections: 16,
        connect_timeout: 5,
        idle_timeout: None,
    };

    let limits = Limits::default()
        .limit("bytes", 25.mebibytes()); // image upload as bytes

    let figment = rocket::Config::figment()
        .merge(("port", context.config.server_port))
        .merge(("limits", limits))
        .merge(("databases.pantsu_db", db_config));

    let _rocket = rocket::custom(figment)
        .mount("/api", routes::get_routes())
        .manage(context)
        .attach(PantsuDB::init())
        .attach(AdHoc::try_on_ignite("DB Migrations", db::migrate))
        .launch()
        .await?;

    Ok(())
}
