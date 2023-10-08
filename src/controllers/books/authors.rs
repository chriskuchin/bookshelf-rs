use crate::{models::books::authors::list_authors as retrieve_authors, AppState};

use super::AppConfig;
use aws_sdk_s3::Client;
use axum::{extract::State, routing::get, Json, Router};
use sqlx::SqlitePool;

pub fn get_routes() -> Router<(SqlitePool, Client, AppConfig)> {
    Router::new().route("/", get(list_authors))
}

async fn list_authors(State(state): State<AppState>) -> Json<Vec<String>> {
    Json(retrieve_authors(&state.db_pool).await)
}
