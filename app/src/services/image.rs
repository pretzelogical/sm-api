use actix_multipart::form::bytes::Bytes as MpBytes;
// use futures_util::StreamExt as _;
// use futures_util::Stream;
use tokio::fs;
// use tokio::io::AsyncWriteExt as _;
use crate::error::AppError;
use crate::routes::post::CreatePostForm;
use crate::FILE_UPLOAD_ROOT;
use uuid::Uuid;

fn is_valid_content_type(content_type: &str) -> bool {
    content_type == "image/jpeg" ||
        content_type == "image/png"
}

pub enum FileConcern {
    Post()
}


// Creates the file path and url for a new image
fn create_image_path(concern: FileConcern, bytes: &MpBytes) -> Result<(String, String), AppError> {
    let name = Uuid::new_v4().to_string();
    let content_type = bytes.content_type.as_ref().unwrap().to_string();
    let image_type = content_type.split("/").last().unwrap_or("");
    if is_valid_content_type(&content_type) {
        match concern {
            FileConcern::Post() => {
                Ok((
                    format!("{}/post/{}.{}", FILE_UPLOAD_ROOT, name, image_type),
                    format!("http://localhost:8080/media/post/{}.{}", name, image_type)
                ))
            }
        }
    } else {
        Err(AppError::BadRequest("Invalid image type"))
    }
}

async fn write_file(concern: FileConcern, bytes: &MpBytes) -> Result<String, AppError> {
    match concern {
        FileConcern::Post() => {
            let (file_path, file_url) = create_image_path(FileConcern::Post(), &bytes)?;
            let file = fs::write(file_path, &bytes.data).await;
            match file {
                Ok(_) => Ok(file_url),
                Err(_) => Err(AppError::InternalError("Failed to write file")),
            }
        }
    }
}


pub async fn upload_post_img(form: &CreatePostForm) -> Result<Option<String>, AppError> {
    match &form.img {
        Some(bytes) => {
            let file_url = write_file(FileConcern::Post(), bytes).await?;
            Ok(Some(file_url))
        },
        None => {
            Ok(None)
        }
    }
}