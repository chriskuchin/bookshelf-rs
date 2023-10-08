use super::AppConfig;
use aws_sdk_s3::Client;
use std::collections::HashMap;

use crate::models::mime::{ext_to_mime, list_mimes as mimes};
use axum::{extract::Path, http::StatusCode, routing::get, Json, Router};
use sqlx::SqlitePool;

pub fn get_routes() -> Router<(SqlitePool, Client, AppConfig)> {
    Router::new()
        .route("/:ext", get(get_mime))
        .route("/", get(list_mimes))
}

pub async fn get_mime(Path(ext): Path<String>) -> Result<Json<String>, StatusCode> {
    match ext_to_mime(&ext) {
        Some(mime) => Ok(Json(mime)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn list_mimes() -> Json<HashMap<String, String>> {
    Json(mimes())
}
