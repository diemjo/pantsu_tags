use rocket::{Route, State};
use rocket::response::content::RawJson;
use rocket_db_pools::Connection;
use tracing::{info, Instrument};

use crate::{Context, Services};
use crate::common::result;
use crate::common::result::Result;
use crate::log::TracingSpan;
use crate::server::PantsuDB;

pub fn get_routes() -> Vec<Route> {
    return rocket::routes![
        get_images
    ]
}

#[rocket::get("/images")]
pub(crate) async fn get_images(context: &State<Context>, services: &State<Services>, span: TracingSpan, db: Connection<PantsuDB>) -> Result<RawJson<String>> {
    let _entered = span.0.enter();
    get_images_impl(context, services, db).in_current_span().await
}

async fn get_images_impl(context: &Context, services: &Services, db: Connection<PantsuDB>) -> Result<RawJson<String>> {
    let statement = db.prepare("").await?;
    let _result = db.query(&statement, &[]).await?;
    // println!("{:?}", result.first().unwrap().get::<usize, u32>(0));
    let sauce = services.iqdb_service.get_sauce("Aqua".to_string()).await?;
    info!("the sauce of {} is {}", "Aqua", sauce);
    Ok(result::wrap_ok(vec![sauce]))
}
