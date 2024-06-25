use sqlx::Mssql;
use sqlx::Pool;
use std::env;

pub async fn establish_connection() -> Pool<Mssql> {
    dotenv::dotenv().expect("Failed to load .env file");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");

    Pool::<Mssql>::connect(&database_url)
        .await
        .expect("Failed to create pool")
}