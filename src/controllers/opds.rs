use super::Message;
use axum::Json;

pub async fn get_opds() -> Json<Message> {
    Json(Message {
        msg: format!("get_opds"),
    })
}
