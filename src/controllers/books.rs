use super::Message;
use crate::controllers::books::files::get_routes as file_routes;
use axum::{extract::Path, routing::get, Json, Router};

pub mod files;

pub fn get_routes() -> Router {
    Router::new()
        .route("/", get(get_books).post(create_book))
        .route("/:id", get(get_book).put(update_book).delete(delete_book))
        .nest("/:id/files", file_routes())
}

async fn get_book(Path(book_id): Path<String>) -> Json<Message> {
    Json(Message {
        msg: format!("get_book -> {}", book_id),
    })
}

async fn update_book(Path(book_id): Path<String>) -> Json<Message> {
    Json(Message {
        msg: format!("update_book -> {}", book_id),
    })
}

async fn delete_book(Path(book_id): Path<String>) -> Json<Message> {
    Json(Message {
        msg: format!("delete_book -> {}", book_id),
    })
}

async fn get_books() -> Json<Message> {
    Json(Message {
        msg: format!("get_books"),
    })
}

async fn create_book() -> Json<Message> {
    Json(Message {
        msg: format!("create_book"),
    })
}
