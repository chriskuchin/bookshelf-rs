use super::Message;
use axum::{extract::Path, routing::get, Json, Router};

pub fn get_routes() -> Router {
    Router::new().route("/:ext", get(get_file).delete(delete_file).post(upload_file))
}

pub async fn get_file(Path((book_id, ext)): Path<(String, String)>) -> Json<Message> {
    Json(Message {
        msg: format!("get_file -> {}, {}", book_id, ext),
    })
}

pub async fn delete_file(Path((book_id, ext)): Path<(String, String)>) -> Json<Message> {
    Json(Message {
        msg: format!("delete_file -> {}, {}", book_id, ext),
    })
}

pub async fn upload_file(Path((book_id, ext)): Path<(String, String)>) -> Json<Message> {
    Json(Message {
        msg: format!("upload_file -> {}, {}", book_id, ext),
    })
}
