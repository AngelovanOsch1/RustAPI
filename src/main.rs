use sqlx::Mssql;
use actix_web::{App, HttpServer, Responder, web, HttpResponse};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct SignupData {
    username: String,
    email: String,
    password: String,
}

#[actix_web::post("/signup")]
async fn signup(signup_data: web::Json<SignupData>) -> impl Responder {
    // Log the incoming signup data
    println!("Received signup request: {:?}", signup_data);

    // Example SQL operation (replace with your actual database logic)
    // For demonstration, you would typically save the data to your database here
    // For now, just return a success message
    HttpResponse::Ok().body("Signup successful!")
}

#[actix_web::get("/greet")]
async fn greet() -> impl Responder {
    // Dummy implementation just to demonstrate another endpoint
    HttpResponse::Ok().body("Hello, world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Ensure your connection string specifies 'charset=utf-8'
    let connection_string = "mssql://sa:wachtwoord123@localhost/chatdatabase?encrypt=true&trustServerCertificate=true&charset=utf-8";
    let pool = sqlx::Pool::<Mssql>::connect(connection_string)
        .await
        .expect("Failed to create pool");

    let port = 8080;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(signup)
            .service(greet)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
