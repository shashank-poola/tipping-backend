use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{NaiveDateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Creator {
    pub id: i32,
    pub username: String,
    pub display_name: String,
    pub email: String,
    pub bio: Option<String>,
    pub profile_image: Option<String>,
    pub wallet_address: Option<String>,
    pub created_at: NaiveDateTime,
}

impl Creator {
    pub fn new(username: &str, display_name: &str, email: &str, wallet: Option<&str>, bio: Option<&str>, profile_image: Option<&str>) -> Self {
        Self {
            id: 0,
            username: username.to_string(),
            display_name: display_name.to_string(),
            email: email.to_string(),
            bio: bio.map(|b| b.to_string()),
            profile_image: profile_image.map(|img| img.to_string()),
            wallet_address: wallet.map(|w| w.to_string()),
            created_at: Utc::now().naive_utc(),
        }
    }
}