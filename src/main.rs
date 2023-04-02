use crate::controllers::get_routes;
use sqlx::sqlite::SqlitePool;
use std::net::SocketAddr;

pub mod controllers;
pub mod models;

#[tokio::main]
async fn main() {
    let port: u16 = option_env!("NOMAD_PORT_run")
        .unwrap_or("")
        .parse::<u16>()
        .unwrap_or(3000);

    // BOOKSHELF_DB_URL
    // BOOKSHELF_STORAGE_URL
    // BOOKSHELF_AWS_ACCESS_KEY_ID
    // BOOKSHELF_AWS_SECRET_ACCESS_KEY
    // BOOKSHELF_AWS_S3_REGION=auto
    // BOOKSHELF_AWS_S3_ENDPOINT_URL

    let pool = SqlitePool::connect("sqlite://bookshelf.db?mode=rwc")
        .await
        .unwrap();

    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(get_routes(pool).into_make_service())
        .await
        .unwrap();
}
