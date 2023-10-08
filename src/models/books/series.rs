use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct Series {
    #[serde(skip_deserializing)]
    id: i64,

    pub name: String,
    pub count: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BookSeries {
    pub name: String,
    pub total: u8,
    pub current: u8,
}

const LIST_SERIES_QUERY: &str = "SELECT * FROM series ORDER BY name";
pub async fn list_series(pool: &SqlitePool) -> Vec<(String, String)> {
    let mut rows = sqlx::query(LIST_SERIES_QUERY).fetch(pool);

    let mut authors: Vec<(String, String)> = Vec::new();

    while let Some(row) = rows.try_next().await.unwrap() {
        authors.push((row.try_get(0).unwrap(), row.try_get(1).unwrap()));
    }

    return authors;
}

const CREATE_SERIES_QUERY: &str = "INSERT into series (id, name, count) VALUES(?, ?, ?)";
pub async fn insert_series(pool: &SqlitePool, series: Series) -> Result<(), sqlx::Error> {
    match sqlx::query(CREATE_SERIES_QUERY)
        .bind(uuid::Uuid::new_v4().to_string())
        .bind(series.name)
        .bind(series.count)
        .execute(pool)
        .await
    {
        Ok(..) => Ok(()),
        Err(err) => Err(err),
    }
}

const GET_BOOK_SERIES_INFO_QUERY: &str = "SELECT series.name, series.count, book_series.series_number FROM book_series JOIN series ON series.id == book_series.series_id WHERE book_series.book_id = ?";
pub async fn get_book_series_info(pool: &SqlitePool, book_id: String) -> Option<BookSeries> {
    match sqlx::query(GET_BOOK_SERIES_INFO_QUERY)
        .bind(book_id)
        .fetch_one(pool)
        .await
    {
        Ok(row) => {
            let name: String = row.try_get(0).unwrap();
            let count: u8 = row.try_get(1).unwrap();
            let number: u8 = row.try_get(2).unwrap();

            Some(BookSeries {
                name: name,
                total: count,
                current: number,
            })
        }
        Err(..) => None,
    }
}

const _UPDATE_SERIES_QUERY: &str = "UPDATE series SET name = ? WHERE series_id = ?";
