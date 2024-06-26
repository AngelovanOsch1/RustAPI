use actix_web::{web, HttpResponse, get, HttpRequest};
use crate::services::user_service::fetch_all_users;
use crate::utils::jwt::decode_jwt; // Import your decode_jwt function
use sqlx::Mssql;

#[get("/users")]
pub async fn get_users(req: HttpRequest, db_pool: web::Data<sqlx::Pool<Mssql>>) -> HttpResponse {
    // Extract the Authorization header
    let auth_header = req.headers().get("Authorization");
    if auth_header.is_none() {
        return HttpResponse::Unauthorized().body("Missing Authorization header");
    }

    // Remove "Bearer " prefix from the token if present
    let token = auth_header.unwrap().to_str().unwrap().replace("Bearer ", "");

    // Decode the JWT token to get the user ID
    let claims = match decode_jwt(&token) {
        Ok(claims) => claims,
        Err(_) => return HttpResponse::Unauthorized().body("Invalid token"),
    };

    // Print the claims to the console
    println!("Decoded Claims: {:?}", claims);

    // Fetch all users except the current user
    match fetch_all_users(db_pool.get_ref(), claims.sub).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error retrieving users: {}", e)),
    }
}
