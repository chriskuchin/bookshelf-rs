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
