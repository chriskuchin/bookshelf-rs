use std::collections::HashMap;

use chrono::{serde::ts_milliseconds_option, DateTime, Utc};
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqliteRow, Row, SqlitePool};
use uuid::Uuid;

use crate::models::books::files::{get_files_by_book_id, File};

use self::series::{get_book_series_info, BookSeries};

pub mod authors;
pub mod files;
pub mod series;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct Book {
    #[serde(skip_deserializing)]
    id: i64,
    #[serde(
        with = "ts_milliseconds_option",
        skip_serializing_if = "Option::is_none",
        skip_deserializing
    )]
    created_at: Option<DateTime<Utc>>,
    #[serde(
        with = "ts_milliseconds_option",
        skip_serializing_if = "Option::is_none",
        skip_deserializing
    )]
    updated_at: Option<DateTime<Utc>>,
    #[serde(
        with = "ts_milliseconds_option",
        skip_serializing_if = "Option::is_none",
        skip_deserializing
    )]
    deleted_at: Option<DateTime<Utc>>,
    #[serde(skip_deserializing)]
    pub uuid: String,
    isbn: String,
    pub title: String,
    author: String,
    description: String,
    cover_url: String,
    publisher: String,
    pub_date: String,
    #[serde(skip_deserializing)]
    pub files: Option<Vec<File>>,

    #[serde(skip_serializing_if = "Option::is_none", skip_deserializing)]
    pub series: Option<BookSeries>,
}

fn row_to_book(row: SqliteRow) -> Book {
    let id: i64 = row.try_get(0).unwrap();
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
        id,
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
        files: None,
        series: None,
    }
}

const COUNT_BOOKS_QUERY: &str = "SELECT COUNT(1) FROM books";
const _COUNT_AUTHORS_QUERY: &str = "SELECT COUNT(DISTINCT author) FROM books";
const _COUNT_FILES_QUERY: &str = "SELECT COUNT(1) FROM files";
const _COUNT_FILE_TYPE_QUERY: &str = "SELECT mime_type, count(1) FROM files GROUP BY mime_type LIMIT 10";
const COUNT_AUTHOR_BOOKS_QUERY: &str = "SELECT author, count(1) FROM books GROUP BY author order by count(1) desc LIMIT 10";
pub async fn get_bookshelf_stats(pool: &SqlitePool) {
    let mut rows = sqlx::query(COUNT_BOOKS_QUERY).fetch(pool);

    while let Some(row) = rows.try_next().await.unwrap() {
        let book_count: i32 = row.try_get("COUNT(1)").unwrap_or(0);
        println!("bookCount: {}", book_count);
    }

    let mut rows = sqlx::query(COUNT_AUTHOR_BOOKS_QUERY).fetch(pool);
    while let Some(row) = rows.try_next().await.unwrap() {
        let author: &str = row.try_get("author").unwrap_or("unknown");
        let count: i32 = row.try_get("count(1)").unwrap_or(0);

        println!("{} {}", author, count);
    }
}


const GET_BOOK_BY_ID_QUERY: &str = "SELECT * FROM books WHERE id = ?";
pub async fn get_book_by_id(pool: &SqlitePool, id: &String) -> Option<Book> {
    let mut rows = sqlx::query(GET_BOOK_BY_ID_QUERY)
        .bind(id)
        .map(row_to_book)
        .fetch(pool);

    while let Some(row) = rows.try_next().await.unwrap() {
        let files = get_files_by_book_id(&pool, row.id.to_string()).await;
        return Some(Book {
            id: row.id,
            created_at: None,
            updated_at: None,
            deleted_at: None,
            uuid: row.uuid.clone(),
            isbn: row.isbn,
            title: row.title,
            author: row.author,
            description: row.description,
            cover_url: row.cover_url,
            publisher: row.publisher,
            pub_date: row.pub_date,
            files: Some(files),
            series: get_book_series_info(&pool, row.uuid).await,
        });
    }

    return None;
}

pub async fn list_books(
    pool: &SqlitePool,
    sort: String,
    limit: u32,
    offset: u32,
    filter: HashMap<String, String>,
) -> Vec<Book> {
    let mut query = format!("SELECT * FROM books LEFT JOIN book_series ON book_id == uuid");

    let mut first_loop = true;
    let mut values: Vec<String> = Vec::new();
    for (field, value) in filter {
        if first_loop {
            query = format!("{} WHERE", query);
            first_loop = false;
        }

        if field == "author" {
            query = format!("{} author like ?", query);
            values.push(value);
        } else if field == "title" {
            query = format!("{} title like ?", query);
            values.push(format!("%{}%", value));
        } else if field == "series" {
            query = format!("{} series_id = ?", query);
            values.push(value);
        }
    }

    query = format!("{} ORDER BY {} ASC LIMIT ? OFFSET ?", query, sort);
    // println!("{}", query);

    let mut query_exec = sqlx::query(&query);

    for val in values {
        query_exec = query_exec.bind(val);
    }

    let mut rows = query_exec
        .bind(limit)
        .bind(offset)
        .map(row_to_book)
        .fetch(pool);

    let mut results: Vec<Book> = Vec::new();
    while let Some(row) = rows.try_next().await.unwrap() {
        let files = get_files_by_book_id(pool, row.id.to_string()).await;
        results.push(Book {
            id: row.id,
            created_at: None,
            updated_at: None,
            deleted_at: None,
            uuid: row.uuid.clone(),
            isbn: row.isbn,
            title: row.title,
            author: row.author,
            description: row.description,
            cover_url: row.cover_url,
            publisher: row.publisher,
            pub_date: row.pub_date,
            files: Some(files),
            series: get_book_series_info(pool, row.uuid).await,
        })
    }

    results
}

const DELETE_BOOK_BY_ID_QUERY: &str = "DELETE FROM books WHERE id = ?";
pub async fn delete_book_by_id(pool: &SqlitePool, book_id: String) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(DELETE_BOOK_BY_ID_QUERY)
        .bind(book_id)
        .execute(pool)
        .await;

    match result {
        Ok(val) => Ok(val.rows_affected()),
        Err(err) => Err(err),
    }
}

const UPDATE_BOOK_BY_ID_QUERY: &str = "UPDATE books SET isbn = ?, title = ?, author = ?, description = ?, cover_url = ?, publisher = ?, pub_date = ? WHERE id = ?";
pub async fn update_book_by_id(
    pool: &SqlitePool,
    book_id: String,
    book: Book,
) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(UPDATE_BOOK_BY_ID_QUERY)
        .bind(book.isbn)
        .bind(book.title)
        .bind(book.author)
        .bind(book.description)
        .bind(book.cover_url)
        .bind(book.publisher)
        .bind(book.pub_date)
        .bind(book_id)
        .execute(pool)
        .await;

    match result {
        Ok(val) => Ok(val.rows_affected()),
        Err(err) => Err(err),
    }
}

const INSERT_BOOK_QUERY: &str = "INSERT INTO books (created_at, updated_at, deleted_at, uuid, isbn, title, author, description, cover_url, publisher, pub_date) VALUES(NULL, NULL, NULL, ?, ?, ?, ?, ?, ?, ?, ?)";
pub async fn insert_book(pool: &SqlitePool, book: Book) -> Result<i64, sqlx::Error> {
    let result = sqlx::query(INSERT_BOOK_QUERY)
        .bind(Uuid::new_v4().to_string())
        .bind(book.isbn)
        .bind(book.title)
        .bind(book.author)
        .bind(book.description)
        .bind(book.cover_url)
        .bind(book.publisher)
        .bind(book.pub_date)
        .execute(pool)
        .await;

    match result {
        Ok(val) => Ok(val.last_insert_rowid()),
        Err(err) => Err(err),
    }
}

const GET_BOOK_TITLE_BY_BOOK_ID: &str = "SELECT title FROM books where id = ?";
pub async fn get_book_title_by_book_id(pool: SqlitePool, book_id: String) -> Option<String> {
    match sqlx::query(GET_BOOK_TITLE_BY_BOOK_ID)
        .bind(book_id)
        .fetch_one(&pool)
        .await
    {
        Ok(row) => Some(row.try_get(0).unwrap()),
        Err(_) => None,
    }
}
