use axum::extract::DefaultBodyLimit;
use axum::Router;
use axum::routing::{get, post};
use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};
use tower_http::trace::TraceLayer;
use crate::AppState;

pub mod image;
pub mod images;
pub mod sauce;
pub mod tags;

pub fn get_router(app_state: AppState) -> Router {
    let config = &app_state.config;
    Router::new()
        .route("/image", get(image::dummy_get_image))
        .route("/image/import", post(image::import)
            .layer(DefaultBodyLimit::max(config.request_body_limit.as_u64() as usize))
        )
        .route("/image/tags", get(image::dummy_get_tags))
        .route("/images", get(images::get_images))
        .with_state(app_state.clone())
        .layer(TraceLayer::new_for_http()
            .make_span_with(crate::log::request_id::request_id_tracing_span)
        )
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid::default()))
        .layer(PropagateRequestIdLayer::x_request_id())
}
