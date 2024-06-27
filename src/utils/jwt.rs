use jsonwebtoken::{encode, decode, Header, Validation, DecodingKey, EncodingKey};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};
use std::env;
use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref TOKEN_STORE: Mutex<HashMap<i32, String>> = Mutex::new(HashMap::new());
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub exp: usize,
    pub iat: usize,
}

impl Claims {
    pub fn new(user_id: i32, expiration: i64) -> Self {
        let now = Utc::now().timestamp() as usize;
        Claims {
            sub: user_id,
            exp: (Utc::now() + Duration::seconds(expiration)).timestamp() as usize,
            iat: now,
        }
    }
}

pub fn generate_access_token(user_id: i32) -> Result<String, Box<dyn std::error::Error>> {
    let access_token_expiration = 15 * 60; // 15 minutes
    let claims = Claims::new(user_id, access_token_expiration);

    let secret = env::var("JWT_ACCESS_TOKEN_SECRET")
        .expect("JWT_ACCESS_TOKEN_SECRET must be set");
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))?;
    Ok(token)
}

pub fn generate_refresh_token(user_id: i32) -> Result<String, Box<dyn std::error::Error>> {
    let refresh_token_expiration = 7 * 24 * 60 * 60; // 7 days
    let claims = Claims::new(user_id, refresh_token_expiration);

    let secret = env::var("JWT_REFRESH_TOKEN_SECRET")
        .expect("JWT_REFRESH_TOKEN_SECRET must be set");
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))?;
    Ok(token)
}

pub fn decode_access_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_ACCESS_TOKEN_SECRET")
        .expect("JWT_ACCESS_TOKEN_SECRET must be set");
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(jsonwebtoken::Algorithm::HS256),
    )?;
    Ok(token_data.claims)
}

pub fn decode_refresh_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_REFRESH_TOKEN_SECRET")
        .expect("JWT_REFRESH_TOKEN_SECRET must be set");
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(jsonwebtoken::Algorithm::HS256),
    )?;
    Ok(token_data.claims)
}

pub fn verify_and_refresh_token(
    incoming_token: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let claims = decode_refresh_token(incoming_token)?;

    let user_id = claims.sub;

    let current_time = Utc::now().timestamp() as usize;
    if claims.exp < current_time {
        return Err("Token has expired".into());
    }

    let mut store = TOKEN_STORE.lock().unwrap();
    if let Some(stored_token) = store.get(&user_id) {
        if stored_token == incoming_token {
            let new_access_token = generate_access_token(user_id)?;
            store.insert(user_id, new_access_token.clone());
            return Ok(new_access_token);
        }
    }

    Err("Invalid token".into())
}
