use actix_multipart::Multipart;
use actix_web::{put, HttpRequest, HttpResponse};
use futures_util::stream::StreamExt as _;

#[put("/editProfile")]
pub async fn save_file(
    req: HttpRequest,
    mut payload: Multipart,
) -> HttpResponse {
    // Log the request headers
    println!("Request headers: {:?}", req.headers());

    // Process the multipart payload and log information
    while let Some(item) = payload.next().await {
        match item {
            Ok(mut field) => {
                let content_disposition = field.content_disposition();
                let filename = content_disposition.get_filename().unwrap_or("file").to_string();
                println!("Filename: {}", filename);
                println!("Field name: {:?}", content_disposition.get_name());

                while let Some(chunk) = field.next().await {
                    match chunk {
                        Ok(data) => {
                            println!("Chunk size: {}", data.len());
                        }
                        Err(e) => {
                            println!("Error reading chunk: {}", e);
                            return HttpResponse::InternalServerError().body(e.to_string());
                        }
                    }
                }
            }
            Err(e) => {
                println!("Error processing field: {}", e);
                return HttpResponse::InternalServerError().body(e.to_string());
            }
        }
    }

    HttpResponse::Ok().body("File received successfully")
}
