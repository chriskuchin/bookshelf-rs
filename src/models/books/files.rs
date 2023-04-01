use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(sqlx::FromRow, Debug, Serialize, Clone)]
pub struct File {
    _id: u64,
    _created_at: DateTime<Utc>,
    _updated_at: DateTime<Utc>,
    _deleted_at: DateTime<Utc>,
    _book_id: u64,
    _mime_type: String,
    _path: String,
    _link: String,
}
