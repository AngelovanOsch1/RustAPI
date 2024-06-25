mod controllers;
mod services;
mod repositories;
mod config;
mod errors;
mod utils;

use actix_web::{web, App, HttpServer};
use crate::config::database::establish_connection;
use crate::controllers::auth_controllers::{signup, login};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().expect("Failed to load .env file");

    let pool = establish_connection().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(signup)
            .service(login)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}