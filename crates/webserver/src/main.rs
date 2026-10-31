mod constants;
mod error;
mod images;
mod pages;
mod ws;

// ---------------------------------------------------------------------------------------------- //

pub(crate) use crate::constants::{
    ADDRESS, HELLO_ROUTE, IMAGE_ROUTE, MAX_IMAGE_BYTES, VIEWER_DIR, VIEWER_ROUTE, WS_ROUTE,
};

// ---------------------------------------------------------------------------------------------- //

use crate::{error::result::AppResult, images::push_image, pages::hello::hello, ws::ws_handler};

use axum::{Router, extract::DefaultBodyLimit, routing};
use tokio::{net::TcpListener, sync::watch};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> AppResult<()> {
    let (images, _) = watch::channel(None);

    let app = Router::new()
        .route(HELLO_ROUTE, routing::get(hello))
        .route(
            IMAGE_ROUTE,
            routing::post(push_image).layer(DefaultBodyLimit::max(MAX_IMAGE_BYTES)),
        )
        .route(WS_ROUTE, routing::get(ws_handler))
        .nest_service(VIEWER_ROUTE, ServeDir::new(VIEWER_DIR))
        .with_state(images);

    let listener = TcpListener::bind(ADDRESS).await?;
    println!("Server started at http://{}", listener.local_addr()?);

    axum::serve(listener, app).await?;

    Ok(())
}
