use crate::controllers::books::get_routes as book_routes;
use crate::controllers::mime::get_routes as mime_routes;
use crate::controllers::opds::get_opds;
use axum::{routing::get, Router};
use serde::Serialize;
use tower_http::services::ServeDir;

pub mod books;
pub mod mime;
pub mod opds;

pub fn get_routes() -> Router {
    Router::new()
        .nest_service("/", ServeDir::new("dist"))
        .route("/opds", get(get_opds))
        .nest(
            "/api/v1",
            Router::new()
                .nest("/books", book_routes())
                .nest("/mimes", mime_routes()),
        )
}

#[derive(Debug, Serialize, Clone)]
pub struct Message {
    msg: String,
}
