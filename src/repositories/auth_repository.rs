use sqlx::Mssql;

pub async fn insert_credentials(pool: &sqlx::Pool<Mssql>, username: &str, password: &str) -> Result<i32, sqlx::Error> {
    let row: (i32,) = sqlx::query_as("INSERT INTO login_credentials (username, password) OUTPUT INSERTED.user_id VALUES (@p1, @p2)")
        .bind(username)
        .bind(password)
        .fetch_one(pool)
        .await?;
    
    Ok(row.0)
}

pub async fn insert_information(pool: &sqlx::Pool<Mssql>, user_id: i32, email: &str) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO user_information (user_id, email) VALUES (@p1, @p2)")
        .bind(user_id)
        .bind(email)
        .execute(pool)
        .await?;
    
    Ok(())
}

pub async fn get_user_credentials(pool: &sqlx::Pool<Mssql>, username: &str) -> Result<Option<(i32, String)>, sqlx::Error> {
    let row: Option<(i32, String)> = sqlx::query_as("SELECT user_id, password FROM login_credentials WHERE username = @p1")
        .bind(username)
        .fetch_optional(pool)
        .await?;

    Ok(row)
}