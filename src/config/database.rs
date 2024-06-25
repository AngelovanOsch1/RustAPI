use sqlx::Mssql;
use sqlx::Pool;
use std::env;

pub async fn establish_connection() -> Pool<Mssql> {
    // Load environment variables from .env file
    dotenv::dotenv().expect("Failed to load .env file");

    // Retrieve DATABASE_URL from environment variables
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");

    // Establish SQLx pool connection
    Pool::<Mssql>::connect(&database_url)
        .await
        .expect("Failed to create pool")
}