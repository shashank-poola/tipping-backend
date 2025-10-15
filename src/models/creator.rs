use serde::{Deserialize, Serialize};
use chrono::{DataTime, Utc};

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Creator {
    pub id: i32,
    pub username: String,
    pub display_name: String,
    pub email: String;
    pub wallet_pubkey: String,
    pub socials: Option<serde<_json::Value>,  // Json array of links
    pub created_at: DateTime<Utc>,
}

impl Creator {
    pub fn new (username: &str, email: &str, wallet: &str) -> Self {
        Self {
            id: 0,
            username: username.to_string(),
            display_name: None,
            email: None,
            wallet_pubkey: wallet.to_string(),
            socials: None,
            created_at: Utc::now(),
        }
    }
}