use super::AppConfig;
use crate::{
    models::{
        books::files::{delete_file_by_book_id, get_file_path_by_mime, insert_file},
        books::{get_book_by_id, get_book_title_by_book_id},
        mime::{ext_to_mime, mime_to_ext},
    },
    AppState,
};
use aws_sdk_s3::types::{CompletedMultipartUpload, CompletedPart};
use aws_sdk_s3::{primitives::ByteStream, Client};
use axum::{
    body::Body,
    extract::{Multipart, Path, State},
    http::HeaderMap,
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use bytes::{BufMut, BytesMut};
use http::{header, StatusCode};
use sqlx::SqlitePool;
use uuid::Uuid;

pub fn get_routes() -> Router<(SqlitePool, Client, AppConfig)> {
    Router::new()
        .route("/", post(batch_file_upload))
        .route("/:ext", get(get_file).delete(delete_file))
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

const CHUNK_SIZE: u64 = 1024 * 1024 * 32;

pub async fn batch_file_upload(
    State(state): State<AppState>,
    Path(book_id): Path<String>,
    header: HeaderMap,
    mut multipart: Multipart,
) -> Response {
    let file_size: u64 = header
        .get("Content-Length")
        .unwrap()
        .to_str()
        .unwrap()
        .parse::<u64>()
        .unwrap();
    println!("Content-Length: {}", file_size);
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
                    let mut chunk_index: u64 = 1;
                    let mut upload_parts: Vec<CompletedPart> = Vec::new();

                    let mut buf = BytesMut::with_capacity(CHUNK_SIZE as usize * 3);
                    while let Some(chunk) = field
                        .chunk()
                        .await
                        .map_err(|err| {
                            println!("{}", err);
                            StatusCode::BAD_REQUEST
                        })
                        .unwrap()
                    {
                        buf.put(chunk);

                        if buf.len() >= CHUNK_SIZE.try_into().unwrap() {
                            let mut batch = buf.split_to(CHUNK_SIZE as usize);
                            println!("Uploading Chunk: {} / {}", batch.len(), buf.len());
                            upload_parts.push(
                                upload_multipart_chunk(
                                    &state.storage_client,
                                    upload_id.as_str(),
                                    ByteStream::from(bytes::Bytes::from(batch.split())),
                                    chunk_index as i32,
                                    state.settings.storage_url.as_str(),
                                    key.as_str(),
                                )
                                .await,
                            );
                            chunk_index += 1;
                        }
                    }

                    upload_parts.push(
                        upload_multipart_chunk(
                            &state.storage_client,
                            upload_id.as_str(),
                            ByteStream::from(bytes::Bytes::from(buf.split())),
                            chunk_index as i32,
                            state.settings.storage_url.as_str(),
                            key.as_str(),
                        )
                        .await,
                    );

                    let completed_multipart_upload: CompletedMultipartUpload =
                        CompletedMultipartUpload::builder()
                            .set_parts(Some(upload_parts))
                            .build();

                    state
                        .storage_client
                        .complete_multipart_upload()
                        .bucket(state.settings.storage_url.as_str())
                        .key(key.as_str())
                        .multipart_upload(completed_multipart_upload)
                        .upload_id(upload_id)
                        .send()
                        .await
                        .unwrap();
                }
            }
            return (StatusCode::OK).into_response();
        }
        None => return (StatusCode::PRECONDITION_REQUIRED).into_response(),
    }
}

async fn upload_multipart_chunk(
    client: &aws_sdk_s3::Client,
    upload_id: &str,
    stream: ByteStream,
    chunk_index: i32,
    storage_url: &str,
    key: &str,
) -> CompletedPart {
    let upload_part_res = client
        .upload_part()
        .part_number(chunk_index)
        .bucket(storage_url)
        .body(stream)
        .key(key)
        .upload_id(upload_id)
        .send()
        .await
        .unwrap();

    CompletedPart::builder()
        .e_tag(upload_part_res.e_tag.unwrap_or_default())
        .part_number(chunk_index as i32)
        .build()
}
