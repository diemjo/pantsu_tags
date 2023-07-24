use rocket::{Route, State};
use rocket::response::content::RawJson;

use crate::Context;
use crate::common::result;
use crate::common::result::Result;

pub fn get_routes() -> Vec<Route> {
    return rocket::routes![
        get_images
    ]
}

#[rocket::get("/images")]
pub(crate) async fn get_images(context: &State<Context>) -> Result<RawJson<String>> {
    let sauce = context.client.get_sauce("Aqua".to_string()).await?;
    println!("the sauce of {} is {}", "Aqua", sauce);
    Ok(result::wrap_ok(vec![sauce]))
}
