use futures::TryStreamExt;
use sqlx::{Row, SqlitePool};

const LIST_AUTHORS_QUERY: &str = "SELECT author, COUNT(1) FROM books GROUP BY author ORDER BY author";
pub async fn list_authors(pool: &SqlitePool) -> Vec<(String, i8)> {
    let mut rows = sqlx::query(LIST_AUTHORS_QUERY).fetch(pool);

    let mut authors: Vec<(String, i8)> = Vec::new();

    while let Some(row) = rows.try_next().await.unwrap() {
        authors.push((row.try_get(0).unwrap(), row.try_get(1).unwrap()));
    }

    return authors;
}
