use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Creator {
    pub id: i32,
    pub username: String,
    pub display_name: String,
    pub email: String,
    pub bio: Option<String>,
    pub profile_image: Option<String>,
    pub wallet_address: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}


impl Creator {
    pub fn new (username: &str, email: &str, wallet: &str) -> Self {
        Self {
            id: 0,
            username: username.to_string(),
            display_name: String,
            email: None,
            wallet_address: wallet.to_string(),
            created_at: Utc::now(),
        }
    }
}