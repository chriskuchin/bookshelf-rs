use super::AppConfig;
use aws_sdk_s3::Client;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use sqlx::SqlitePool;

pub fn get_routes() -> Router<(SqlitePool, Client, AppConfig)> {
    Router::new().route("/", get(list_series)).route(
        "/:id",
        get(get_series)
            .delete(delete_series)
            .put(update_series)
            .post(create_series),
    )
}

pub async fn list_series() -> Response {
    (StatusCode::NOT_IMPLEMENTED).into_response()
}

pub async fn get_series() -> Response {
    (StatusCode::NOT_IMPLEMENTED).into_response()
}

pub async fn create_series() -> Response {
    (StatusCode::NOT_IMPLEMENTED).into_response()
}

pub async fn update_series() -> Response {
    (StatusCode::NOT_IMPLEMENTED).into_response()
}

pub async fn delete_series() -> Response {
    (StatusCode::NOT_IMPLEMENTED).into_response()
}
