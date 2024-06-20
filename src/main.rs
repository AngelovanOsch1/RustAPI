use sqlx::Mssql;
use actix_web::{App, HttpServer, Responder, web, HttpResponse};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct SignupData {
    username: String,
    email: String,
    password: String,
}

// Function to insert data into the login_credentials table
async fn insert_credentials(pool: &sqlx::Pool<Mssql>, username: &str, password: &str) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO login_credentials (username, password_hash) VALUES (?, ?)")
        .bind(username)
        .bind(password)
        .execute(pool)
        .await?;
    
    Ok(())
}

// Function to insert data into the customer_information table
async fn insert_information(pool: &sqlx::Pool<Mssql>, email: &str) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO customer_information (email) VALUES (?)")
        .bind(email)
        .execute(pool)
        .await?;
    
    Ok(())
}

#[actix_web::post("/signup")]
async fn signup(signup_data: web::Json<SignupData>, db_pool: web::Data<sqlx::Pool<Mssql>>) -> HttpResponse {
    // Log the incoming signup data
    println!("Received signup request: {:?}", signup_data);

    let username = &signup_data.username;
    let password = &signup_data.password;
    let email = &signup_data.email;

    // Insert into login_credentials table
    match insert_credentials(db_pool.get_ref(), username, password).await {
        Ok(_) => {
            // Insert into customer_information table
            match insert_information(db_pool.get_ref(), email).await {
                Ok(_) => HttpResponse::Ok().body("Signup successful!"),
                Err(e) => {
                    eprintln!("Failed to insert into customer_information table: {:?}", e);
                    HttpResponse::InternalServerError().body("Failed to process signup")
                }
            }
        },
        Err(e) => {
            eprintln!("Failed to insert into login_credentials table: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to process signup")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Ensure your connection string specifies 'charset=utf-8'
    let connection_string = "mssql://sa:wachtwoord123@localhost/chatdatabase?encrypt=true&trustServerCertificate=true&charset=utf-8";
    let pool = sqlx::Pool::<Mssql>::connect(connection_string)
        .await
        .expect("Failed to create pool");

    // Specify the port
    let port = 8080;

    // Start Actix server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone())) // Pass the database pool to application state
            .service(signup)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
