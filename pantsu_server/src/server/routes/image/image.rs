use axum::Json;

use crate::common::result::Result;

pub async fn dummy_get_image() -> Result<Json<String>> {
    Ok(Json(String::new()))
}
