use crate::common::result::Result;
use crate::AppState;

// mod forms;
mod multipart;
mod routes;

pub async fn launch_server(shared_state: AppState) -> Result<()> {
    let listener = tokio::net::TcpListener::bind(("0.0.0.0", shared_state.config.server_port)).await?;

    let app = routes::get_router(shared_state);

    axum::serve(listener, app).await?;

    Ok(())
}
