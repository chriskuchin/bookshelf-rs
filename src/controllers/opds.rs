use super::Message;
use axum::{extract::State, Json};
use sqlx::SqlitePool;

pub async fn get_opds(State(_pool): State<SqlitePool>) -> Json<Message> {
    Json(Message {
        msg: format!("get_opds"),
    })
}
