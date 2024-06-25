mod controllers;
mod services;
mod repositories;
mod models;
mod config;
mod errors;
mod utils;

use actix_web::{web, App, HttpServer};
use crate::config::database::establish_connection;
use crate::controllers::auth_controllers::{signup, login};
use crate::controllers::user_controller::get_users;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let pool = establish_connection().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(signup)
            .service(login)
            .service(get_users) // Register the new route
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
