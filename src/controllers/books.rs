use super::{AppConfig, Message, PagingOptions};
use crate::{
    controllers::books::files::get_routes as file_routes,
    models::books::{
        delete_book_by_id, get_book_by_id, insert_book, list_books, update_book_by_id, Book,
    },
};
use aws_sdk_s3::Client;
use axum::{
    extract::{self, Path, Query, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use sqlx::SqlitePool;

pub mod files;

pub fn get_routes() -> Router<(SqlitePool, Client, AppConfig)> {
    Router::new()
        .route("/", get(get_books).post(create_book))
        .route("/:id", get(get_book).put(update_book).delete(delete_book))
        .nest("/:id/files", file_routes())
}

async fn get_book(
    State((pool, _, _settings)): State<(SqlitePool, Client, AppConfig)>,
    Path(book_id): Path<String>,
) -> Result<Json<Book>, StatusCode> {
    match get_book_by_id(&pool, book_id).await {
        Some(book) => Ok(Json(book)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn update_book(
    State((pool, _, _settings)): State<(SqlitePool, Client, AppConfig)>,
    Path(book_id): Path<String>,
    extract::Json(payload): extract::Json<Book>,
) -> Result<(), StatusCode> {
    match update_book_by_id(&pool, book_id, payload).await {
        Ok(val) => {
            if val == 0 {
                return Err(StatusCode::NOT_FOUND);
            }

            Ok(())
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn delete_book(
    State((pool, _, _settings)): State<(SqlitePool, Client, AppConfig)>,
    Path(book_id): Path<String>,
) -> Result<(), StatusCode> {
    match delete_book_by_id(&pool, book_id).await {
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        Ok(val) => {
            if val == 0 {
                return Err(StatusCode::NOT_FOUND);
            }
            Ok(())
        }
    }
}

async fn get_books(
    State((pool, _, _settings)): State<(SqlitePool, Client, AppConfig)>,
    paging: Query<PagingOptions>,
) -> Json<Vec<Book>> {
    Json(
        list_books(
            &pool,
            paging.0.limit.unwrap_or(10),
            paging.0.offset.unwrap_or(0),
        )
        .await,
    )
}

async fn create_book(
    State((pool, _, _settings)): State<(SqlitePool, Client, AppConfig)>,
    extract::Json(payload): extract::Json<Book>,
) -> Result<Json<u64>, StatusCode> {
    let result = insert_book(&pool, payload).await;
    match result {
        Ok(val) => Ok(Json(val)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
