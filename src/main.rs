use crate::controllers::get_routes;
use sqlx::sqlite::SqlitePoolOptions;
use std::net::SocketAddr;

pub mod controllers;

#[tokio::main]
async fn main() {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite://bookshelf.db?mode=rwc")
        .await
        .ok()
        .unwrap();

    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(get_routes().into_make_service())
        .await
        .unwrap();
}
