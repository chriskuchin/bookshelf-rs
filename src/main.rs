use crate::controllers::get_routes;
use aws_sdk_s3::config::{Credentials, Region};
use aws_sdk_s3::{Client, Config as StorageConfig};
use axum::extract::FromRef;
use clap::Parser;
use serde::Deserialize;
use sqlx::sqlite::SqlitePool;

pub mod controllers;
pub mod models;

#[derive(Debug, Clone)]
#[allow(unused)]
pub struct AppState {
    settings: AppConfig,
    db_pool: SqlitePool,
    storage_client: Client,
}

#[derive(Debug, Deserialize, Clone, Parser)]
#[allow(unused)]
pub struct AppConfig {
    #[arg(long, env = "BOOKSHELF_DB_URL")]
    db_url: String,

    #[arg(long, env = "BOOKSHELF_STORAGE_URL")]
    storage_url: String,

    #[arg(long, default_value = "3000", env = "BOOKSHELF_PORT")]
    port: String,

    #[arg(long, env = "BOOKSHELF_AWS_ACCESS_KEY_ID")]
    aws_access_key_id: String,

    #[arg(long, env = "BOOKSHELF_AWS_SECRET_ACCESS_KEY")]
    aws_secret_access_key: String,

    #[arg(long, env = "BOOKSHELF_AWS_S3_REGION")]
    aws_s3_region: String,

    #[arg(long, env = "BOOKSHELF_AWS_S3_ENDPOINT_URL")]
    aws_s3_endpoint_url: String,

    #[arg(long, default_value = "dist", env = "BOOKSHELF_FRONTEND_LOCATION")]
    frontend_location: String,

    #[arg(long, default_value_t = false, env = "BOOKSHELF_DEBUG")]
    debug: bool,
}

#[tokio::main]
async fn main() {
    let settings = AppConfig::parse();

    let port: u16 = option_env!("NOMAD_PORT_run")
        .unwrap_or(settings.port.as_str())
        .parse::<u16>()
        .unwrap();

    let storage_creds = Credentials::new(
        &settings.aws_access_key_id,
        &settings.aws_secret_access_key,
        None,
        None,
        "bookshelf",
    );

    if settings.debug {
        tracing_subscriber::fmt()
            .with_target(false)
            .pretty()
            .compact()
            .init();
    } else {
        tracing_subscriber::fmt().with_target(false).json().init();
    }

    let storage_config = StorageConfig::builder()
        .region(Region::new(settings.aws_s3_region.clone()))
        .endpoint_url(&settings.aws_s3_endpoint_url)
        .credentials_provider(storage_creds)
        .build();

    let storage_client = Client::from_conf(storage_config);

    let pool = SqlitePool::connect(&settings.db_url).await.unwrap();

    sqlx::migrate!().run(&pool).await.unwrap();

    let app = get_routes(pool, storage_client, settings);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

impl FromRef<(SqlitePool, Client, AppConfig)> for AppState {
    fn from_ref(app_state: &(SqlitePool, Client, AppConfig)) -> AppState {
        AppState {
            settings: app_state.2.clone(),
            db_pool: app_state.0.clone(),
            storage_client: app_state.1.clone(),
        }
    }
}
