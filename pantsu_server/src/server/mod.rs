use rocket::data::Limits;
use rocket::fairing::AdHoc;
use rocket_db_pools::Database;

use crate::common::result::Result;
use crate::{Context, db, Services};
use crate::db::PantsuDB;
use crate::log::TracingFairing;

mod forms;
mod routes;

pub async fn launch_server(context: Context, services: Services) -> Result<()> {
    let limits = Limits::default()
        .limit("data-form", context.config.data_form_limit)
        .limit("image-file", context.config.image_file_limit);

    let figment = rocket::Config::figment()
        .merge(("port", context.config.server_port))
        .merge(("limits", limits))
        .merge(("databases.pantsu_db", db::config()));

    let _rocket = rocket::custom(figment)
        .mount("/api", routes::get_routes())
        .manage(context)
        .manage(services)
        .attach(PantsuDB::init())
        .attach(TracingFairing)
        .attach(AdHoc::try_on_ignite("db migrations", db::migrate))
        .launch()
        .await?;

    Ok(())
}
