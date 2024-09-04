use actix_web::{web, HttpResponse, post};
use crate::services::auth_service::{SignupData, LoginData, signup_service, login_service, logout_service};
use sqlx::Mssql;

#[post("/signup")]
pub async fn signup(
    signup_data: web::Json<SignupData>,
    db_pool: web::Data<sqlx::Pool<Mssql>>,
) -> HttpResponse {
    match signup_service(signup_data.into_inner(), db_pool.get_ref()).await {
        Ok(token) => HttpResponse::Ok().json(token),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[post("/login")]
pub async fn login(
    login_data: web::Json<LoginData>,
    db_pool: web::Data<sqlx::Pool<Mssql>>,
) -> HttpResponse {
    match login_service(login_data.into_inner(), db_pool.get_ref()).await {
        Ok(token) => HttpResponse::Ok().json(token),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[post("/logout/{user_id}")]
pub async fn logout(
    user_id: web::Path<u32>,
    db_pool: web::Data<sqlx::Pool<Mssql>>,
) -> HttpResponse {
    let user_id_i32 = user_id.into_inner() as i32;
    match logout_service(user_id_i32, db_pool.get_ref()).await {
        Ok(()) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}