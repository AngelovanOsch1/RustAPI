use sqlx::mssql::MssqlPoolOptions;
use sqlx::Pool;
use sqlx::Mssql;

pub async fn establish_connection(database_url: &str) -> Pool<Mssql> {
    MssqlPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .expect("Failed to create pool.")
}
