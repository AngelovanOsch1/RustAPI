use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use sqlx::Pool;
use sqlx::Mssql;
use std::env;

mod handlers;
mod models;
mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();  // Load .env file
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::establish_connection(&database_url).await;

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::default().allow_any_origin().allow_any_method().allow_any_header())
            .data(pool.clone())
            .route("/items", web::get().to(handlers::get_items))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
