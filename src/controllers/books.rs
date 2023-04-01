use super::{Message, PagingOptions};
use crate::{
    controllers::books::files::get_routes as file_routes,
    models::books::{delete_book_by_id, get_book_by_id, list_books, Book},
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use sqlx::SqlitePool;

pub mod files;

pub fn get_routes() -> Router<SqlitePool> {
    Router::new()
        .route("/", get(get_books).post(create_book))
        .route("/:id", get(get_book).put(update_book).delete(delete_book))
        .nest("/:id/files", file_routes())
}

async fn get_book(
    State(pool): State<SqlitePool>,
    Path(book_id): Path<String>,
) -> Result<Json<Book>, StatusCode> {
    match get_book_by_id(pool, book_id).await {
        Some(book) => Ok(Json(book)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn update_book(
    State(_pool): State<SqlitePool>,
    Path(book_id): Path<String>,
) -> Json<Message> {
    Json(Message {
        msg: format!("update_book -> {}", book_id),
    })
}

async fn delete_book(
    State(pool): State<SqlitePool>,
    Path(book_id): Path<String>,
) -> Result<StatusCode, ()> {
    match delete_book_by_id(pool, book_id).await {
        Err(_) => Err(()),
        _ => Ok(StatusCode::NO_CONTENT),
    }
}

async fn get_books(
    State(pool): State<SqlitePool>,
    paging: Query<PagingOptions>,
) -> Json<Vec<Option<Book>>> {
    Json(
        list_books(
            pool,
            paging.0.limit.unwrap_or(10),
            paging.0.offset.unwrap_or(0),
        )
        .await,
    )
}

async fn create_book(State(_pool): State<SqlitePool>) -> Json<Message> {
    Json(Message {
        msg: format!("create_book"),
    })
}
