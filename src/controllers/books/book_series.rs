use super::AppConfig;
use aws_sdk_s3::Client;
use axum::{
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use http::StatusCode;
use sqlx::SqlitePool;

pub fn get_routes() -> Router<(SqlitePool, Client, AppConfig)> {
    Router::new().route("/", get(list_series).post(create_series))
}

pub async fn list_series() -> Response {
    (StatusCode::NOT_IMPLEMENTED).into_response()
}

pub async fn create_series() -> Response {
    (StatusCode::NOT_IMPLEMENTED).into_response()
}

pub async fn update_series() -> Response {
    (StatusCode::NOT_IMPLEMENTED).into_response()
}
