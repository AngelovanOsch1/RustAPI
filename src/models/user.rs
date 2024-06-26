use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub profile_photo: Option<String>,
    pub profile_banner: Option<String>,
    pub is_online: bool,
    pub role: String,
}
