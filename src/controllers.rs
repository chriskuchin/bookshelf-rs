use crate::controllers::books::get_routes as book_routes;
use crate::controllers::mime::get_routes as mime_routes;
use crate::controllers::opds::get_opds;
use axum::{routing::get, Router};
use http::Method;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;

pub mod books;
pub mod mime;
pub mod opds;

pub fn get_routes(pool: SqlitePool) -> Router<()> {
    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST, Method::PUT])
        // allow requests from any origin
        .allow_origin(Any);

    Router::new()
        .nest_service("/", ServeDir::new("dist"))
        .route("/opds", get(get_opds))
        .nest(
            "/api/v1",
            Router::new()
                .nest("/books", book_routes())
                .nest("/mimes", mime_routes()),
        )
        .with_state(pool)
        .layer(ServiceBuilder::new().layer(cors))
}

#[derive(Debug, Serialize, Clone)]
pub struct Message {
    msg: String,
}

#[derive(Debug, Deserialize)]
struct PagingOptions {
    limit: Option<u32>,
    offset: Option<u32>,
}
