use actix_web::{web, HttpResponse, post};
use crate::services::auth_service::{SignupData, LoginData, signup_service, login_service};
use sqlx::Mssql;

#[post("/signup")]
pub async fn signup(signup_data: web::Json<SignupData>, db_pool: web::Data<sqlx::Pool<Mssql>>) -> HttpResponse {
    match signup_service(signup_data.into_inner(), db_pool.get_ref()).await {
        Ok(token) => HttpResponse::Ok().json(token),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[post("/login")]
pub async fn login(login_data: web::Json<LoginData>, db_pool: web::Data<sqlx::Pool<Mssql>>) -> HttpResponse {
    match login_service(login_data.into_inner(), db_pool.get_ref()).await {
        Ok(token) => HttpResponse::Ok().json(token),
        Err(e) => HttpResponse::Unauthorized().body(e.to_string()),
    }
}