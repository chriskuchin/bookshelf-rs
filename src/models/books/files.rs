use chrono::{DateTime, Utc};
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqliteRow, Row, SqlitePool};

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct File {
    pub id: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<DateTime<Utc>>,
    pub book_id: u32,
    #[serde(rename = "type")]
    pub mime_type: String,
    pub path: String,
}

fn row_to_file(row: SqliteRow) -> File {
    let id: u32 = row.try_get(0).unwrap();
    // let created_at: &str = row.try_get(1).unwrap();
    // let updated_at: &str = row.try_get(2).unwrap();
    // let deleted_at: &str = row.try_get(3).unwrap();
    let book_id: u32 = row.try_get(4).unwrap();
    let mime_type: String = row.try_get(5).unwrap();
    let path: String = row.try_get(6).unwrap();

    File {
        id: id,
        created_at: None,
        updated_at: None,
        deleted_at: None,
        book_id: book_id,
        mime_type: mime_type,
        path: path,
    }
}

const GET_FILES_BY_BOOK_ID_QUERY: &str = "SELECT * FROM files where book_id = ?";
pub async fn get_files_by_book_id(pool: &SqlitePool, book_id: String) -> Vec<File> {
    let mut rows = sqlx::query(GET_FILES_BY_BOOK_ID_QUERY)
        .bind(book_id)
        .map(row_to_file)
        .fetch(pool);

    let mut results: Vec<File> = Vec::new();
    while let Some(row) = rows.try_next().await.unwrap() {
        results.push(row)
    }

    results
}
const GET_FILE_PATH_BY_MIME_QUERY: &str =
    "SELECT path FROM files where book_id = ? and mime_type in (?, ?)";
pub async fn get_file_path_by_mime(
    pool: &SqlitePool,
    book_id: &String,
    mime: &String,
    ext: &String,
) -> Option<String> {
    let result = sqlx::query(GET_FILE_PATH_BY_MIME_QUERY)
        .bind(book_id)
        .bind(mime)
        .bind(ext)
        .fetch_one(pool)
        .await;

    match result {
        Ok(row) => Some(row.try_get(0).unwrap()),
        Err(_) => None,
    }
}
