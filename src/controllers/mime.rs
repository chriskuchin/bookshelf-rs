use std::collections::HashMap;

use crate::models::mime::{ext_to_mime, list_mimes as mimes};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use sqlx::SqlitePool;

pub fn get_routes() -> Router<SqlitePool> {
    Router::new()
        .route("/:ext", get(get_mime))
        .route("/", get(list_mimes))
}

pub async fn get_mime(
    State(_pool): State<SqlitePool>,
    Path(ext): Path<String>,
) -> Result<Json<String>, StatusCode> {
    match ext_to_mime(ext) {
        Some(mime) => Ok(Json(mime)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn list_mimes(State(_pool): State<SqlitePool>) -> Json<HashMap<String, String>> {
    Json(mimes())
}
