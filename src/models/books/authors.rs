use futures::TryStreamExt;
use sqlx::{Row, SqlitePool};

const LIST_AUTHORS_QUERY: &str = "SELECT DISTINCT author FROM books ORDER BY author";
pub async fn list_authors(pool: &SqlitePool) -> Vec<String> {
    let mut rows = sqlx::query(LIST_AUTHORS_QUERY).fetch(pool);

    let mut authors: Vec<String> = Vec::new();

    while let Some(row) = rows.try_next().await.unwrap() {
        authors.push(row.try_get(0).unwrap());
    }

    return authors;
}
