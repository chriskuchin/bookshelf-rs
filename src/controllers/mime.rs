use super::Message;
use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use sqlx::SqlitePool;

pub fn get_routes() -> Router<SqlitePool> {
    Router::new()
        .route("/:ext", get(get_mime))
        .route("/", get(list_mimes))
}

pub async fn get_mime(State(_pool): State<SqlitePool>, Path(ext): Path<String>) -> Json<Message> {
    Json(Message {
        msg: format!("get_mime -> {}", ext),
    })
}

pub async fn list_mimes(State(_pool): State<SqlitePool>) -> Json<Message> {
    Json(Message {
        msg: format!("list_mimes"),
    })
}
