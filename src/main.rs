use crate::controllers::get_routes;
use aws_sdk_s3::config::{Credentials, Region};
use aws_sdk_s3::{Client, Config as StorageConfig};
use config::Config;
use serde::Deserialize;
use sqlx::sqlite::SqlitePool;
use std::net::SocketAddr;

pub mod controllers;
pub mod models;

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct AppConfig {
    db_url: String,
    storage_url: String,
    port: u16,
    aws_access_key_id: String,
    aws_secret_access_key: String,
    aws_s3_region: String,
    aws_s3_endpoint_url: String,
}

#[tokio::main]
async fn main() {
    let port: u16 = option_env!("NOMAD_PORT_run")
        .unwrap_or("")
        .parse::<u16>()
        .unwrap_or(3000);

    let settings = Config::builder()
        .set_default("port", port)
        .unwrap()
        .add_source(config::File::with_name("config"))
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .add_source(config::Environment::with_prefix("BOOKSHELF"))
        .build()
        .unwrap()
        .try_deserialize::<AppConfig>()
        .unwrap();

    let storage_creds = Credentials::new(
        &settings.aws_access_key_id,
        &settings.aws_secret_access_key,
        None,
        None,
        "bookshelf",
    );

    let storage_config = StorageConfig::builder()
        .region(Region::new(settings.aws_s3_region.clone()))
        .endpoint_url(&settings.aws_s3_endpoint_url)
        .credentials_provider(storage_creds)
        .build();

    let storage_client = Client::from_conf(storage_config);

    let pool = SqlitePool::connect(&settings.db_url).await.unwrap();

    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    let addr = SocketAddr::from(([0, 0, 0, 0], settings.port));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(get_routes(pool, storage_client, settings).into_make_service())
        .await
        .unwrap();
}

// BOOKSHELF_DB_URL
// BOOKSHELF_STORAGE_URL
// BOOKSHELF_AWS_ACCESS_KEY_ID
// BOOKSHELF_AWS_SECRET_ACCESS_KEY
// BOOKSHELF_AWS_S3_REGION=auto
// BOOKSHELF_AWS_S3_ENDPOINT_URL
