use super::{AppConfig, Message};
use aws_sdk_s3::Client;
use axum::{extract::State, Json};
use sqlx::SqlitePool;

pub async fn get_opds(
    State((_pool, _storage, _settings)): State<(SqlitePool, Client, AppConfig)>,
) -> Json<Message> {
    Json(Message {
        msg: format!("get_opds"),
    })
}
