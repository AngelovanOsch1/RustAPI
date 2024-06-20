use sqlx::Mssql;
use sqlx::Pool;
use actix_web::{App, HttpServer, web, HttpResponse};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct SignupData {
    username: String,
    email: String,
    password: String,
}

async fn insert_credentials(pool: &Pool<Mssql>, username: &str, password: &str) -> Result<i32, sqlx::Error> {
    let row: (i32,) = sqlx::query_as("INSERT INTO login_credentials (username, password) OUTPUT INSERTED.user_id VALUES (@p1, @p2)")
        .bind(username)
        .bind(password)
        .fetch_one(pool)
        .await?;
    
    Ok(row.0)
}

async fn insert_information(pool: &sqlx::Pool<Mssql>, user_id: i32, email: &str) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO user_information (user_id, email) VALUES (@p1, @p2)")
        .bind(user_id)
        .bind(email)
        .execute(pool)
        .await?;
    
    Ok(())
}

#[actix_web::post("/signup")]
async fn signup(signup_data: web::Json<SignupData>, db_pool: web::Data<sqlx::Pool<Mssql>>) -> HttpResponse {
    println!("Received signup request: {:?}", signup_data);

    let username = &signup_data.username;
    let password = &signup_data.password;
    let email = &signup_data.email;

    match insert_credentials(db_pool.get_ref(), username, password).await {
        Ok(user_id) => {
            match insert_information(db_pool.get_ref(), user_id, email).await {
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
    let connection_string = "mssql://sa:wachtwoord123@localhost/chatdatabase?encrypt=true&trustServerCertificate=true&charset=utf-8";
    let pool = sqlx::Pool::<Mssql>::connect(connection_string)
        .await
        .expect("Failed to create pool");

    let port = 8080;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(signup)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
