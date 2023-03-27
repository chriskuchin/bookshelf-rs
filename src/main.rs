use crate::controllers::get_routes;
use std::net::SocketAddr;

pub mod controllers;

#[tokio::main]
async fn main() {
    // run it
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(get_routes().into_make_service())
        .await
        .unwrap();
}
