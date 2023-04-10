use super::{AppConfig, Message};
use crate::models::{
    books::files::{get_file_path_by_mime, File},
    books::get_book_title_by_book_id,
    mime::ext_to_mime,
};
use aws_sdk_s3::Client;
use axum::{
    body::Full,
    extract::{Multipart, Path, State},
    response::{IntoResponse, Response},
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
    State((pool, storage, settings)): State<(SqlitePool, Client, AppConfig)>,
    Path((book_id, ext)): Path<(String, String)>,
) -> Response {
    match ext_to_mime(&ext) {
        Some(mime) => match get_file_path_by_mime(&pool, &book_id, &mime).await {
            Some(path) => {
                let file = storage
                    .get_object()
                    .bucket(settings.storage_url.clone())
                    .key(path.clone())
                    .send()
                    .await
                    .unwrap();

                let title = get_book_title_by_book_id(pool, book_id)
                    .await
                    .unwrap_or(String::from("unknown"));

                let attachment_header = format!("attachment; filename = \"{}.{}\"", title, ext);
                let headers = [
                    (header::CONTENT_TYPE, mime.as_str()),
                    (header::CONTENT_DISPOSITION, attachment_header.as_str()),
                ];

                return (
                    headers,
                    Full::from(file.body.collect().await.unwrap().to_vec()),
                )
                    .into_response();
            }
            None => {}
        },
        None => {}
    }

    (StatusCode::NOT_FOUND).into_response()
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
            mime_type: ext_to_mime(&ext).unwrap(),
            path: format!("{}/{}/{}", file_name, name, "other"),
        }));
    }

    return Err(StatusCode::NOT_ACCEPTABLE);
}
