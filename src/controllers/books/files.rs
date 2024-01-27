use super::AppConfig;
use crate::{
    models::{
        books::files::{delete_file_by_book_id, get_file_path_by_mime, insert_file},
        books::{get_book_by_id, get_book_title_by_book_id},
        mime::{ext_to_mime, mime_to_ext},
    },
    AppState,
};
use aws_sdk_s3::{primitives::ByteStream, Client};
use axum::{
    body::Body,
    extract::{Multipart, Path, State},
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use http::{header, StatusCode};
use sqlx::SqlitePool;
use uuid::Uuid;

pub fn get_routes() -> Router<(SqlitePool, Client, AppConfig)> {
    Router::new()
        .route("/", post(batch_file_upload))
        .route("/:ext", get(get_file).delete(delete_file).post(upload_file))
}

pub async fn get_file(
    State(state): State<AppState>,
    Path((book_id, ext)): Path<(String, String)>,
) -> Response {
    match ext_to_mime(&ext) {
        Some(..) => match get_file_path_by_mime(&state.db_pool, &book_id, &ext).await {
            Some(path) => {
                let file = state
                    .storage_client
                    .get_object()
                    .bucket(state.settings.storage_url.clone())
                    .key(path.clone())
                    .send()
                    .await
                    .unwrap();

                let title = get_book_title_by_book_id(state.db_pool, book_id)
                    .await
                    .unwrap_or(String::from("unknown"));

                let attachment_header = format!("attachment; filename = \"{}.{}\"", title, ext);
                let headers = [
                    // (header::CONTENT_TYPE, mime.as_str()),
                    (header::CONTENT_DISPOSITION, attachment_header.as_str()),
                ];

                return (
                    headers,
                    Body::from(file.body.collect().await.unwrap().to_vec()),
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
    State(state): State<AppState>,
    Path((book_id, ext)): Path<(String, String)>,
) -> Response {
    let key = get_file_path_by_mime(&state.db_pool, &book_id, &ext)
        .await
        .unwrap();

    match state
        .storage_client
        .delete_object()
        .bucket(state.settings.storage_url)
        .key(key)
        .send()
        .await
    {
        Ok(_) => {
            if delete_file_by_book_id(&state.db_pool, &book_id, &ext).await {
                return (StatusCode::OK).into_response();
            }
        }
        Err(err) => println!("{}", err),
    }

    (StatusCode::BAD_REQUEST).into_response()
}

pub async fn batch_file_upload(
    State(state): State<AppState>,
    Path(book_id): Path<String>,
    mut multipart: Multipart,
) -> Response {
    match get_book_by_id(&state.db_pool, &book_id).await {
        Some(book) => {
            while let Some(mut field) = multipart.next_field().await.unwrap() {
                let file_name = field.file_name().unwrap_or("unknown_file").to_string();
                let ext = field.name().unwrap_or("unknown_name").to_string();
                // let data = field.bytes().await.unwrap();
                // // println!("{} {} {}", file_name, name, data.len());
                println!("{}, {}", ext, file_name);

                let key = format!("{}/{}/{}", book.clone().uuid, ext.clone(), Uuid::new_v4());

                for file in book.clone().files.unwrap_or_default() {
                    if mime_to_ext(&file.mime_type) == ext {
                        return (StatusCode::CONFLICT).into_response();
                    }
                }

                if insert_file(&state.db_pool, &book_id, &ext, &key).await {
                    let res = state
                        .storage_client
                        .create_multipart_upload()
                        .bucket(state.settings.storage_url.as_str())
                        .key(key.as_str())
                        .set_content_type(ext_to_mime(&ext))
                        .send()
                        .await
                        .unwrap();

                    let upload_id = res.upload_id.unwrap();
                    let mut part_num = 1;
                    while let Some(chunk) = field
                        .chunk()
                        .await
                        .map_err(|err| {
                            println!("{}", err);
                            StatusCode::BAD_REQUEST
                        })
                        .unwrap()
                    {
                        println!("chunk of bytes {}", chunk.len());

                        state
                            .storage_client
                            .upload_part()
                            .part_number(part_num)
                            .bucket(state.settings.storage_url.as_str())
                            .body(ByteStream::from(chunk))
                            .key(key.as_str())
                            .upload_id(upload_id.clone())
                            .send()
                            .await
                            .unwrap();

                        part_num += 1;
                    }

                    state
                        .storage_client
                        .complete_multipart_upload()
                        .bucket(state.settings.storage_url.as_str())
                        .key(key.as_str())
                        .upload_id(upload_id.clone())
                        .send()
                        .await
                        .unwrap();

                    // match state
                    //     .storage_client
                    //     .put_object()
                    //     .bucket(state.settings.storage_url.as_str())
                    //     .body(ByteStream::from(data))
                    //     .key(key.as_str())
                    //     .set_content_type(ext_to_mime(&ext))
                    //     .send()
                    //     .await
                    // {
                    //     Ok(_) => continue,
                    //     Err(err) => {
                    //         println!("Failed to upload, {}", err);
                    //         return (StatusCode::BAD_REQUEST).into_response();
                    //     }
                    // }
                }
            }
            return (StatusCode::OK).into_response();
        }
        None => return (StatusCode::PRECONDITION_REQUIRED).into_response(),
    }
}

pub async fn upload_file(
    State(state): State<AppState>,
    Path((book_id, ext)): Path<(String, String)>,
    mut multipart: Multipart,
) -> Response {
    match get_book_by_id(&state.db_pool, &book_id).await {
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

            if insert_file(&state.db_pool, &book_id, &ext, &key).await {
                while let Some(field) = multipart.next_field().await.unwrap() {
                    // let file_name = field.file_name().unwrap_or("unknown_file").to_string();
                    // let name = field.name().unwrap_or("unknown_name").to_string();
                    let data = field.bytes().await.unwrap();
                    // println!("{} {} {}", file_name, name, data.len());

                    match state
                        .storage_client
                        .put_object()
                        .bucket(state.settings.storage_url.as_str())
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
