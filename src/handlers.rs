use actix_web::{web, HttpResponse};
use sqlx::Pool;
use sqlx::Mssql;

use crate::models::Item;

pub async fn get_items(db_pool: web::Data<Pool<Mssql>>) -> HttpResponse {
    let items = sqlx::query_as!(Item, "SELECT id, name FROM items")
        .fetch_all(db_pool.get_ref())
        .await
        .expect("Error fetching items");

    HttpResponse::Ok().json(items)
}
