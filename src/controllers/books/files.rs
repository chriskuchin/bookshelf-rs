use super::Message;
use crate::models::{books::files::File, mime::ext_to_mime};
use axum::{
    extract::{Multipart, Path, State},
    routing::get,
    Json, Router,
};
use http::StatusCode;
use sqlx::SqlitePool;
use std::{fs::File as FSFile, io::Write};

pub fn get_routes() -> Router<SqlitePool> {
    Router::new().route("/:ext", get(get_file).delete(delete_file).post(upload_file))
}

pub async fn get_file(
    State(_pool): State<SqlitePool>,
    Path((book_id, ext)): Path<(String, String)>,
) -> Json<Message> {
    Json(Message {
        msg: format!("get_file -> {}, {}", book_id, ext),
    })
}

pub async fn delete_file(
    State(_pool): State<SqlitePool>,
    Path((book_id, ext)): Path<(String, String)>,
) -> Json<Message> {
    Json(Message {
        msg: format!("delete_file -> {}, {}", book_id, ext),
    })
}

pub async fn upload_file(
    State(_pool): State<SqlitePool>,
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
