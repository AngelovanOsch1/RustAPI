use sqlx::Mssql;
use crate::models::user::User;

pub async fn get_all_users(pool: &sqlx::Pool<Mssql>, current_user_id: i32) -> Result<Vec<User>, sqlx::Error> {
    let users = sqlx::query_as!(
        User,
        r#"
        SELECT 
            login_credentials.user_id as id,
            login_credentials.username,
            user_information.email
        FROM 
            login_credentials
        INNER JOIN 
            user_information 
        ON 
            login_credentials.user_id = user_information.user_id
        WHERE 
            login_credentials.user_id != @p1;
        "#,
        current_user_id as i32
    )
    .fetch_all(pool)
    .await?;

    Ok(users)
}
