use super::AppConfig;
use crate::controllers::books::get_routes as book_routes;
use crate::controllers::mime::get_routes as mime_routes;
use crate::controllers::opds::get_opds;
use crate::controllers::series::get_routes as series_routes;
use aws_sdk_s3::Client;
use axum::extract::DefaultBodyLimit;
use axum::response::IntoResponse;
use axum::{routing::get, Router};

use http::Method;
use http::StatusCode;

use serde::Serialize;
use sqlx::SqlitePool;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

pub mod books;
pub mod mime;
pub mod opds;
pub mod series;

pub fn get_routes(pool: SqlitePool, storage_client: Client, settings: AppConfig) -> Router<()> {
    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        // allow requests from any origin
        .allow_origin(Any);

    Router::new()
        .nest_service(
            "/",
            ServeDir::new(settings.clone().frontend_location)
                .not_found_service(ServeFile::new("dist/index.html")),
        )
        .route("/opds", get(get_opds))
        .nest(
            "/api/v1",
            Router::new()
                .nest("/books", book_routes())
                .nest("/series", series_routes())
                .nest("/mimes", mime_routes())
                .fallback(handler_404),
        )
        .with_state((pool, storage_client, settings))
        .layer(ServiceBuilder::new().layer(cors))
        .layer(DefaultBodyLimit::disable())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}

#[derive(Debug, Serialize, Clone)]
pub struct Message {
    msg: String,
}
