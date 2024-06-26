use serde::Deserialize;
use sqlx::Mssql;
use crate::repositories::auth_repository::{insert_credentials, insert_information, get_user_credentials};
use crate::utils::jwt::generate_jwt;
use argon2::{self, Config};
use rand::Rng;

#[derive(Debug, Deserialize)]
pub struct SignupData {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginData {
    pub username: String,
    pub password: String,
}

pub async fn signup_service(signup_data: SignupData, pool: &sqlx::Pool<Mssql>) -> Result<String, Box<dyn std::error::Error>> {
    let username = &signup_data.username;
    let plaintext_password = &signup_data.password;
    let email = &signup_data.email;

    let config = Config::default();
    
    let mut rng = rand::thread_rng();
    let mut salt = [0u8; 16];
    rng.fill(&mut salt);
    
    let hashed_password = argon2::hash_encoded(plaintext_password.as_bytes(), &salt, &config)?;

    let user_id = insert_credentials(pool, username, &hashed_password).await?;
    insert_information(pool, user_id, email).await?;

    let token = generate_jwt(user_id)?;
    Ok(token)
}

pub async fn login_service(login_data: LoginData, pool: &sqlx::Pool<Mssql>) -> Result<String, Box<dyn std::error::Error>> {
    let username = &login_data.username;
    let password = &login_data.password;

    let (user_id, hashed_password) = match get_user_credentials(pool, username).await? {
        Some(credentials) => credentials,
        None => return Err("Invalid credentials".into()),
    };

    if !argon2::verify_encoded(&hashed_password, password.as_bytes()).unwrap_or(false) {
        return Err("Invalid credentials".into());
    }

    let token = generate_jwt(user_id)?;
    Ok(token)
}