use super::Message;
use axum::{extract::Path, routing::get, Json, Router};

pub fn get_routes() -> Router {
    Router::new()
        .route("/:ext", get(get_mime))
        .route("/", get(list_mimes))
}

pub async fn get_mime(Path(ext): Path<String>) -> Json<Message> {
    Json(Message {
        msg: format!("get_mime -> {}", ext),
    })
}

pub async fn list_mimes() -> Json<Message> {
    Json(Message {
        msg: format!("list_mimes"),
    })
}
