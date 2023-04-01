use crate::controllers::get_routes;
use sqlx::sqlite::SqlitePool;
use std::net::SocketAddr;

pub mod controllers;
pub mod models;

#[tokio::main]
async fn main() {
    let pool = SqlitePool::connect("sqlite://bookshelf.db?mode=rwc")
        .await
        .unwrap();

    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    let addr = SocketAddr::from(([0, 0, 0, 0], 31526));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(get_routes(pool).into_make_service())
        .await
        .unwrap();
}
