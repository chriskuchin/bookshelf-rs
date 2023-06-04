use super::AppConfig;
use crate::models::{
    books::files::{delete_file_by_book_id, get_file_path_by_mime, insert_file},
    books::{get_book_by_id, get_book_title_by_book_id},
    mime::{ext_to_mime, mime_to_ext},
};
use aws_sdk_s3::{primitives::ByteStream, Client};
use axum::{
    body::Full,
    extract::{Multipart, Path, State},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use http::{header, StatusCode};
use sqlx::SqlitePool;
use uuid::Uuid;

pub fn get_routes() -> Router<(SqlitePool, Client, AppConfig)> {
    Router::new().route("/:ext", get(get_file).delete(delete_file).post(upload_file))
}

pub async fn get_file(
    State((pool, storage, settings)): State<(SqlitePool, Client, AppConfig)>,
    Path((book_id, ext)): Path<(String, String)>,
) -> Response {
    match ext_to_mime(&ext) {
        Some(mime) => match get_file_path_by_mime(&pool, &book_id, &ext).await {
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
                println!("{} \n {}", attachment_header, mime.as_str());
                let headers = [
                    // (header::CONTENT_TYPE, mime.as_str()),
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
    State((pool, storage, settings)): State<(SqlitePool, Client, AppConfig)>,
    Path((book_id, ext)): Path<(String, String)>,
) -> Response {
    let key = get_file_path_by_mime(&pool, &book_id, &ext).await.unwrap();

    match storage
        .delete_object()
        .bucket(settings.storage_url)
        .key(key)
        .send()
        .await
    {
        Ok(_) => {
            if delete_file_by_book_id(&pool, &book_id, &ext).await {
                return (StatusCode::OK).into_response();
            }
        }
        Err(err) => println!("{}", err),
    }

    (StatusCode::BAD_REQUEST).into_response()
}

pub async fn upload_file(
    State((pool, storage, settings)): State<(SqlitePool, Client, AppConfig)>,
    Path((book_id, ext)): Path<(String, String)>,
    mut multipart: Multipart,
) -> Response {
    match get_book_by_id(&pool, &book_id).await {
        // verify book exists
        Some(book) => {
            // generate s3 file name... sha hash of file?
            let key = format!("{}/{}/{}", &book.uuid, &ext, Uuid::new_v4());

            if book.files.is_some() {
                for file in book.files.unwrap() {
                    if mime_to_ext(&file.mime_type) == ext {
                        return (StatusCode::CONFLICT).into_response();
                    }
                }
            }

            if ext_to_mime(&ext).is_none() {
                return (StatusCode::BAD_REQUEST).into_response();
            }

            if insert_file(&pool, &book_id, &ext, &key).await {
                while let Some(field) = multipart.next_field().await.unwrap() {
                    // let file_name = field.file_name().unwrap_or("unknown_file").to_string();
                    // let name = field.name().unwrap_or("unknown_name").to_string();
                    let data = field.bytes().await.unwrap();
                    // println!("{} {} {}", file_name, name, data.len());

                    match storage
                        .put_object()
                        .bucket(settings.storage_url.as_str())
                        .body(ByteStream::from(data))
                        .key(key.as_str())
                        .set_content_type(ext_to_mime(&ext))
                        .send()
                        .await
                    {
                        Ok(_) => continue,
                        Err(err) => println!("Failed to upload, {}", err),
                    }
                }
            }
        }
        None => return (StatusCode::NOT_FOUND).into_response(),
    }

    return (StatusCode::ACCEPTED).into_response();
}
