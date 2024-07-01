use sqlx::{Mssql, Pool};

use crate::models::user::User;

pub async fn get_all_users(pool: &Pool<Mssql>, current_user_id: i32) -> Result<Vec<User>, sqlx::Error> {
    let users = sqlx::query_as!(
        User,
        r#"
        SELECT 
            lc.user_id as id,
            lc.username,
            ui.email,
            CONVERT(VARCHAR(MAX), ui.profile_photo, 1) AS profile_photo, -- Convert VARBINARY to Base64 encoded string
            CONVERT(VARCHAR(MAX), ui.profile_banner, 1) AS profile_banner, -- Convert VARBINARY to Base64 encoded string
            ui.is_online,
            ui.role
        FROM 
            login_credentials lc
        INNER JOIN 
            user_information ui
        ON 
            lc.user_id = ui.user_id
        WHERE 
            lc.user_id != @p1;
        "#,
        current_user_id
    )
    .fetch_all(pool)
    .await?;

    Ok(users)
}
