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
    // Load environment variables from .env file
    dotenv::dotenv().expect("Failed to load .env file");

    // Establish database connection pool
    let pool = establish_connection().await;

    // Start Actix Web server
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
