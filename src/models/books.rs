use chrono::{serde::ts_milliseconds_option, DateTime, Utc};
use futures::TryStreamExt;
use serde::Serialize;
use sqlx::{sqlite::SqliteRow, Row, SqlitePool};
pub mod files;

#[derive(sqlx::FromRow, Debug, Serialize, Clone)]
pub struct Book {
    id: u64,
    #[serde(with = "ts_milliseconds_option")]
    created_at: Option<DateTime<Utc>>,
    #[serde(with = "ts_milliseconds_option")]
    updated_at: Option<DateTime<Utc>>,
    #[serde(with = "ts_milliseconds_option")]
    deleted_at: Option<DateTime<Utc>>,
    uuid: String,
    isbn: String,
    title: String,
    author: String,
    description: String,
    cover_url: String,
    publisher: String,
    pub_date: String,
    // files: [File; 0],
}

const GET_BOOK_BY_ID_QUERY: &str = "SELECT * FROM books where id = ?";
const LIST_BOOKS_QUERY: &str = "SELECT * FROM books LIMIT ? OFFSET ?";
const DELETE_BOOK_BY_ID_QUERY: &str = "DELETE FROM books where id = ?";

fn row_to_book(row: SqliteRow) -> Book {
    let id: u32 = row.try_get(0).unwrap();
    // let created_at: &str = row.try_get(1).unwrap();
    // let updated_at: &str = row.try_get(2).unwrap();
    // let deleted_at: &str = row.try_get(3).unwrap();
    let uuid: String = row.try_get(4).unwrap();
    let isbn: String = row.try_get(5).unwrap();
    let title: String = row.try_get(6).unwrap();
    let author: String = row.try_get(7).unwrap();
    let description: String = row.try_get(8).unwrap();
    let cover_url: String = row.try_get(9).unwrap();
    let publisher: String = row.try_get(10).unwrap();
    let pub_date: String = row.try_get(11).unwrap();

    Book {
        id: id.into(),
        created_at: None,
        updated_at: None,
        deleted_at: None,
        uuid,
        isbn,
        title,
        author,
        description,
        cover_url,
        publisher,
        pub_date,
    }
}

pub async fn get_book_by_id(pool: SqlitePool, id: String) -> Option<Book> {
    let mut rows = sqlx::query(GET_BOOK_BY_ID_QUERY)
        .bind(id)
        .map(row_to_book)
        .fetch(&pool);

    while let Some(row) = rows.try_next().await.unwrap() {
        return Some(row);
    }

    return None;
}

pub async fn list_books(pool: SqlitePool, limit: u32, offset: u32) -> Vec<Option<Book>> {
    let mut rows = sqlx::query(LIST_BOOKS_QUERY)
        .bind(limit)
        .bind(offset)
        .map(row_to_book)
        .fetch(&pool);

    const INIT: Option<Book> = None;
    let mut results = Vec::new();
    while let Some(row) = rows.try_next().await.unwrap() {
        results.push(Some(row))
    }

    results
}

pub async fn delete_book_by_id(pool: SqlitePool, book_id: String) -> Result<(), sqlx::Error> {
    let result = sqlx::query(DELETE_BOOK_BY_ID_QUERY)
        .bind(book_id)
        .execute(&pool)
        .await;

    match result {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}
