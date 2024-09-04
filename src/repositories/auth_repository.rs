use sqlx::Mssql;
use crate::models::user::User;

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

pub async fn get_user_model(
    pool: &sqlx::Pool<Mssql>,
    user_id: i32,
) -> Result<Option<User>, sqlx::Error> {
    set_user_online(pool, true, user_id).await?;
    
    let user: Option<User> = sqlx::query_as!(
        User,
        r#"
        SELECT 
            lc.user_id AS id,
            lc.username,
            ui.email,
            CAST(ui.profile_photo AS VARCHAR(MAX)) AS profile_photo, -- Convert binary to string
            CAST(ui.profile_banner AS VARCHAR(MAX)) AS profile_banner, -- Convert binary to string
            ui.is_online,
            ui.role
        FROM user_information ui
        INNER JOIN login_credentials lc ON lc.user_id = ui.user_id
        WHERE lc.user_id = @p1
        "#,
        user_id
    )
    .fetch_optional(pool)
    .await?;
    Ok(user)
}

pub async fn set_user_online(
    pool: &sqlx::Pool<Mssql>,
    login_status: bool,
    user_id: i32,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE user_information
        SET is_online = @p1
        WHERE user_id = @p2
        "#,
        login_status,
        user_id
    )
    .execute(pool)
    .await?;

    Ok(())
}
