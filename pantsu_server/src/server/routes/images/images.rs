use axum::Json;
use crate::common::result::Result;

pub async fn get_images() -> Result<Json<String>> {
    Ok(Json(String::new()))
}

// pub(crate) async fn get_images2(context: &State<Context>, services: &State<Services>, span: TracingSpan, db: Connection<PantsuDB>) -> Result<RawJson<String>> {
//     get_images_impl2(context, services, db).instrument(span.0).await
// }
//
// async fn get_images_impl2(context: &Context, services: &Services, db: Connection<PantsuDB>) -> Result<RawJson<String>> {
//     let statement = db.prepare("").await?;
//     let _result = db.query(&statement, &[]).await?;
//     // println!("{:?}", result.first().unwrap().get::<usize, u32>(0));
//     let sauce = services.iqdb_service.get_sauce("Aqua".to_string()).await?;
//     info!("the sauce of {} is {}", "Aqua", sauce);
//     Ok(result::wrap_ok(vec![sauce]))
// }
