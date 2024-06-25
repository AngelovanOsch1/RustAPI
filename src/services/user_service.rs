use crate::{models::user::User, repositories::user_repository};
use sqlx::Mssql;

pub async fn fetch_all_users(pool: &sqlx::Pool<Mssql>, current_user_id: i32) -> Result<Vec<User>, String> {
    user_repository::get_all_users(pool, current_user_id).await.map_err(|e| e.to_string())
}
