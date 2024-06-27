use actix_web::{web, HttpResponse, get, HttpRequest, http::header};
use crate::services::user_service::fetch_all_users;
use crate::utils::jwt::{decode_access_token, verify_and_refresh_token};
use sqlx::Mssql;

#[get("/users")]
pub async fn get_users(req: HttpRequest, db_pool: web::Data<sqlx::Pool<Mssql>>) -> HttpResponse {
    let auth_header = req.headers().get("Authorization");
    if auth_header.is_none() {
        return HttpResponse::Unauthorized().body("Missing Authorization header");
    }

    let token = auth_header.unwrap().to_str().unwrap().replace("Bearer ", "");

    let claims = match decode_access_token(&token) {
        Ok(claims) => claims,
        Err(_) => {
            match verify_and_refresh_token(&token) {
                Ok(new_token) => {
                    return HttpResponse::Ok()
                        .append_header((header::AUTHORIZATION, format!("Bearer {}", new_token)))
                        .body("Access token refreshed");
                }
                Err(_) => return HttpResponse::Unauthorized().body("Invalid token"),
            }
        }
    };

    println!("Decoded Claims: {:?}", claims);

    match fetch_all_users(db_pool.get_ref(), claims.sub).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error retrieving users: {}", e)),
    }
}
