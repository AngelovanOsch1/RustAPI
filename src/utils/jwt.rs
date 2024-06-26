use jsonwebtoken::{encode, decode, Header, Validation, DecodingKey, EncodingKey};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub exp: usize,
}

impl Claims {
    pub fn new(user_id: i32) -> Self {
        let expiration = Utc::now()
            .checked_add_signed(Duration::seconds(3600))
            .expect("valid timestamp")
            .timestamp() as usize;

        Claims {
            sub: user_id,
            exp: expiration,
        }
    }
}

pub fn generate_jwt(user_id: i32) -> Result<String, Box<dyn std::error::Error>> {
    let claims = Claims::new(user_id);

    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))?;
    Ok(token)
}

pub fn decode_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(jsonwebtoken::Algorithm::HS256),
    )?;
    Ok(token_data.claims)
}
