use axum::{
    extract::{Extension, Path, Json},
    http::StatusCode
};

use sqlx::pgPool;
use serde::{
    Deserilaize, Serialize
};
use serde_json::json;
use crate::utils::response::ApiResponse;
use crate::model::creator::Creator;

mod utils;
mod models;

#[derive(Deserialize)]
pub struct CreateCreatorInput {
    pub username: String,
    pub display_name: String,
    pub email: String,
    pub bio: Option<String>,
    pub profile_image: Option<String>,
    pub wallet_address: String,
}

pub async fn create_creator(
    Extension(pool): Extension<pgPool>,
    Json(Payload): Json<CreateCreatorInput>,
) -> (StatusCode, Json<ApiResponse<serde_json::Value>>) {
    let username_exists = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM creators WHERE username = $1",
    )
    .bind(&payload.username)
    .fetch_one(&pool)
    .await
    .unwrap_or(0);


    if username_exists > 0 {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::error("Username already exists".to_string()))
        );
    }

    let result = sqlx::query!(
        "INSERT INTO creators (username, display_name, bio, email, profile_image, wallet_address)
         VALUES ($1, $2, $3, $4, $5) RETURNING id",
        payload.username,
        payload.display_name,
        payload.email,
        payload.bio,
        payload.profile_image,
        payload.wallet_address
    )
    .fetch_one(&pool)
    .await;

    match result {
        Ok(record) => (
            StatusCode::CREATED,
            Json(ApiResponse::success(json!({ "id": record.id })))
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(e.to_string()))       
        ),
    }
}