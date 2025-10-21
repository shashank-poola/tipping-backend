use axum::{
    extract::{Extension, Json, Path},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use chrono::NaiveDateTime;

use crate::utils::response::ApiResponse;

#[derive(Deserialize)]
pub struct CreateTipInput {
    pub creator_id: i32,
    pub sender_wallet: String,
    pub amount: f64,
    pub message: Option<String>,
    pub signature: String,
}

#[derive(Deserialize, Serialize, sqlx::FromRow, Debug)]
pub struct Tip {
    pub id: i32,
    pub creator_id: i32,
    pub sender_wallet: String,
    pub amount: f64,
    pub message: Option<String>,
    pub signature: String,
    pub created_at: NaiveDateTime,
}

pub async fn create_tip(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<CreateTipInput>,
) -> (StatusCode, Json<ApiResponse<serde_json::Value>>) {
    let existing = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM tips WHERE signature = $1"
    )
    .bind(&payload.signature)
    .fetch_one(&pool)
    .await
    .unwrap_or(0);

    if existing > 0 {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::error("Duplicate transaction signature".to_string())),
        );
    }

    let result = sqlx::query!(
        "INSERT INTO tips (creator_id, sender_wallet, amount, message, signature) \
        VALUES ($1, $2, $3, $4, $5) RETURNING id",
        payload.creator_id,
        payload.sender_wallet,
        payload.amount,
        payload.message,
        payload.signature
    )
    .fetch_one(&pool)
    .await;

    match result {
        Ok(record) => (
            StatusCode::CREATED,
            Json(ApiResponse::success(json!({ "id": record.id }))),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(e.to_string())),
        ),
    }
}

pub async fn get_tips_for_creator(
    Extension(pool): Extension<PgPool>,
    Path(creator_id): Path<i32>,
) -> (StatusCode, Json<ApiResponse<Vec<Tip>>>) {
    let result = sqlx::query_as::<_, Tip>(
        "SELECT * FROM tips WHERE creator_id = $1 ORDER BY created_at DESC"
    )
    .bind(creator_id)
    .fetch_all(&pool)
    .await;

    match result {
        Ok(tips) => (
            StatusCode::OK,
            Json(ApiResponse::success(tips)),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(e.to_string())),
        ),
    }
}

pub async fn get_recent_tips(
    Extension(pool): Extension<PgPool>,
) -> (StatusCode, Json<ApiResponse<Vec<Tip>>>) {
    let result = sqlx::query_as::<_, Tip>(
        "SELECT * FROM tips ORDER BY created_at DESC LIMIT 10"
    )
    .fetch_all(&pool)
    .await;

    match result {
        Ok(tips) => (
            StatusCode::OK,
            Json(ApiResponse::success(tips)),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(e.to_string())),
        ),
    }
}