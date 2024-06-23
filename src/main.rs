use actix_web::{web, App, HttpResponse, HttpServer, post};
use serde::{Deserialize, Serialize};
use sqlx::Mssql;
use sqlx::Pool;
use jsonwebtoken::{encode, Header, EncodingKey};
use chrono::Utc;

#[derive(Debug, Deserialize)]
struct SignupData {
    username: String,
    email: String,
    password: String,
}

#[derive(Debug, Deserialize)]
struct LoginData {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
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

async fn get_user_id(pool: &Pool<Mssql>, username: &str, password: &str) -> Result<Option<i32>, sqlx::Error> {
    let row: Option<(i32,)> = sqlx::query_as("SELECT user_id FROM login_credentials WHERE username = @p1 AND password = @p2")
        .bind(username)
        .bind(password)
        .fetch_optional(pool)
        .await?;
    
    Ok(row.map(|r| r.0))
}

fn generate_jwt(user_id: i32, secret: &str) -> Result<String, Box<dyn std::error::Error>> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(3600))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration,
    };

    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))?;
    Ok(token)
}

#[post("/signup")]
async fn signup(signup_data: web::Json<SignupData>, db_pool: web::Data<sqlx::Pool<Mssql>>) -> HttpResponse {
    println!("Received signup request: {:?}", signup_data);

    let username = &signup_data.username;
    let password = &signup_data.password;
    let email = &signup_data.email;

    match insert_credentials(db_pool.get_ref(), username, password).await {
        Ok(user_id) => {
            match insert_information(db_pool.get_ref(), user_id, email).await {
                Ok(_) => {
                    let token = generate_jwt(user_id, "secret").unwrap();
                    HttpResponse::Ok().json(token)
                },
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

#[post("/login")]
async fn login(login_data: web::Json<LoginData>, db_pool: web::Data<sqlx::Pool<Mssql>>) -> HttpResponse {
    println!("Received login request: {:?}", login_data);

    let username = &login_data.username;
    let password = &login_data.password;

    match get_user_id(db_pool.get_ref(), username, password).await {
        Ok(Some(user_id)) => {
            let token = generate_jwt(user_id, "secret").unwrap();
            HttpResponse::Ok().json(token)
        },
        Ok(None) => HttpResponse::Unauthorized().body("Invalid credentials"),
        Err(e) => {
            eprintln!("Failed to verify credentials: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to process login")
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
            .service(login)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}