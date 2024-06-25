use actix_web::{web, HttpResponse, get, HttpRequest};
use crate::services::user_service::fetch_all_users;
use sqlx::Mssql;

#[get("/users")]
pub async fn get_users(req: HttpRequest, db_pool: web::Data<sqlx::Pool<Mssql>>) -> HttpResponse {
    // Assuming you get the current user's ID from the request's headers or some authentication mechanism
    let current_user_id: i32 = get_current_user_id_from_request(&req); // Implement this function based on your auth logic

    match fetch_all_users(db_pool.get_ref(), current_user_id).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error retrieving users: {}", e)),
    }
}

fn get_current_user_id_from_request(req: &HttpRequest) -> i32 {
    1
}