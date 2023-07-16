use rocket::{Route, State};
use rocket::response::content::RawJson;

use crate::common::result;
use crate::common::result::Result;
use crate::config::ServerConfig;

pub fn get_routes() -> Vec<Route> {
    return rocket::routes![
        get_images
    ]
}

#[rocket::get("/images")]
pub(crate) async fn get_images(config: &State<ServerConfig>) -> Result<RawJson<String>> {
    Ok(result::wrap_ok(vec![config.db_username.to_string()]))
}
