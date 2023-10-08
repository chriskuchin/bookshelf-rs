use crate::{
    models::books::series::{insert_series, list_series as get_unique_series},
    AppState,
};

use super::AppConfig;
use aws_sdk_s3::Client;

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};

use crate::models::books::series::Series;
use sqlx::SqlitePool;

pub fn get_routes() -> Router<(SqlitePool, Client, AppConfig)> {
    Router::new()
        .route("/", get(list_series).post(create_series))
        .route(
            "/:id",
            get(get_series).delete(delete_series).put(update_series),
        )
}

pub async fn list_series(State(state): State<AppState>) -> Response {
    Json(get_unique_series(&state.db_pool).await).into_response()
}

pub async fn get_series() -> Response {
    (StatusCode::NOT_IMPLEMENTED).into_response()
}

pub async fn create_series(
    State(state): State<AppState>,
    axum::extract::Json(series): axum::extract::Json<Series>,
) -> Response {
    match insert_series(&state.db_pool, series).await {
        Ok(..) => (StatusCode::OK).into_response(),
        Err(..) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    }
}

// async fn create_book(
//     State(state): State<AppState>,
//     extract::Json(payload): extract::Json<Book>,
// ) -> Result<Json<i64>, StatusCode> {
//     let result = insert_book(&state.db_pool, payload).await;
//     match result {
//         Ok(val) => Ok(Json(val)),
//         Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
//     }
// }

pub async fn update_series() -> Response {
    (StatusCode::NOT_IMPLEMENTED).into_response()
}

pub async fn delete_series() -> Response {
    (StatusCode::NOT_IMPLEMENTED).into_response()
}
