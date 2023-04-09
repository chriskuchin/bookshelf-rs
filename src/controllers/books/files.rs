use super::{AppConfig, Message};
use crate::models::{books::files::File, mime::ext_to_mime};
use aws_sdk_s3::Client;
use axum::{
    body::Full,
    extract::{Multipart, Path, State},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use http::{header, StatusCode};
use sqlx::SqlitePool;
use std::{fs::File as FSFile, io::Write};

pub fn get_routes() -> Router<(SqlitePool, Client, AppConfig)> {
    Router::new().route("/:ext", get(get_file).delete(delete_file).post(upload_file))
}

pub async fn get_file(
    State((_pool, storage, settings)): State<(SqlitePool, Client, AppConfig)>,
    Path((_book_id, _ext)): Path<(String, String)>,
) -> impl IntoResponse {
    let res = storage
        .list_objects()
        .bucket(settings.storage_url.clone())
        .send()
        .await
        .unwrap();

    for obj in res.contents().unwrap() {
        let file = storage
            .get_object()
            .bucket(settings.storage_url.clone())
            .key(obj.key().unwrap().clone())
            .send()
            .await
            .unwrap();

        let headers = [
            (header::CONTENT_TYPE, "application/x-mobipocket-ebook"),
            (
                header::CONTENT_DISPOSITION,
                "attachment; filename=\"book.mobi\"",
            ),
        ];

        return (
            headers,
            Full::from(file.body.collect().await.unwrap().to_vec()),
        );
    }
    return (
        [
            (header::CONTENT_TYPE, "text/plain"),
            (header::LAST_MODIFIED, ""),
        ],
        Full::from("value"),
    );
}

pub async fn delete_file(
    State((_pool, _, _settings)): State<(SqlitePool, Client, AppConfig)>,
    Path((book_id, ext)): Path<(String, String)>,
) -> Json<Message> {
    Json(Message {
        msg: format!("delete_file -> {}, {}", book_id, ext),
    })
}

pub async fn upload_file(
    State((_pool, _storage, _settings)): State<(SqlitePool, Client, AppConfig)>,
    Path((book_id, ext)): Path<(String, String)>,
    mut multipart: Multipart,
) -> Result<Json<File>, StatusCode> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let file_name = field.file_name().unwrap().to_string();
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        // println!("Length of `{}` is {} bytes", name, data.len());

        // println!(
        //     "{} - {}",
        //     file_name,
        //     ext_to_mime("kindle.epub".to_string()).unwrap()
        // );

        // verify book exists
        // ensure non duplicates... outside of mime type
        // generate s3 file name... sha hash of file?
        // upload to s3
        // record file in DB

        let mut f = FSFile::create("test.mobi").unwrap();
        f.write_all(data.to_vec().as_slice()).unwrap();

        return Ok(Json(File {
            id: 100,
            created_at: None,
            updated_at: None,
            deleted_at: None,
            book_id: book_id.parse::<u32>().unwrap(),
            mime_type: ext_to_mime(ext).unwrap(),
            path: format!("{}/{}/{}", file_name, name, "other"),
        }));
    }

    return Err(StatusCode::NOT_ACCEPTABLE);
}
