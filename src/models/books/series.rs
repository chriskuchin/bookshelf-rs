use futures::TryStreamExt;
use sqlx::{Row, SqlitePool};

const LIST_SERIES_QUERY: &str = "SELECT DISTINCT name FROM series ORDER BY name";
pub async fn list_series(pool: &SqlitePool) -> Vec<String> {
    let mut rows = sqlx::query(LIST_SERIES_QUERY).fetch(pool);

    let mut authors: Vec<String> = Vec::new();

    while let Some(row) = rows.try_next().await.unwrap() {
        authors.push(row.try_get(0).unwrap());
    }

    return authors;
}

const _CREATE_SERIES_QUERY: &str = "INSERT into series (name) VALUES(?)";
const _UPDATE_SERIES_QUERY: &str = "UPDATE series SET name = ? WHERE series_id = ?";
