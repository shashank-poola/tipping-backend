use axum::{
    extract::{Extension, Path, Json},
    http::StatusCode,
};
use sqlx::PgPool;
use serde::{Deserialize};
use serde_json::json;

use crate::utils::response::ApiResponse;
use crate::models::creators::Creator;

#[derive(Deserialize)]
pub struct CreateCreatorInput {
    pub username: String,
    pub display_name: String,
    pub email: String,
    pub bio: Option<String>,
    pub profile_image: Option<String>,
    pub wallet_address: Option<String>,
}

// CREATE a new creator
pub async fn create_creator(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<CreateCreatorInput>,
) -> (StatusCode, Json<ApiResponse<serde_json::Value>>) {
    let username_exists: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM creators WHERE username = $1")
        .bind(&payload.username)
        .fetch_one(&pool)
        .await
        .unwrap_or(0);
    if username_exists > 0 {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::error("Username already exists".to_string())),
        );
    }
    let result = sqlx::query!(
        "INSERT INTO creators (username, display_name, email, bio, profile_image, wallet_address)
         VALUES ($1, $2, $3, $4, $5, $6) RETURNING id",
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
            Json(ApiResponse::success(json!({ "id": record.id }))),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(e.to_string())),
        ),
    }
}

// LIST all creators
pub async fn list_creators(
    Extension(pool): Extension<PgPool>,
) -> (StatusCode, Json<ApiResponse<Vec<Creator>>>) {
    let result = sqlx::query_as::<_, Creator>("SELECT * FROM creators")
        .fetch_all(&pool)
        .await;
    match result {
        Ok(creators) => (
            StatusCode::OK,
            Json(ApiResponse::success(creators)),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(e.to_string())),
        ),
    }
}

// GET a creator by ID
pub async fn get_creator(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
) -> (StatusCode, Json<ApiResponse<Creator>>) {
    let result = sqlx::query_as::<_, Creator>("SELECT * FROM creators WHERE id = $1")
        .bind(id)
        .fetch_one(&pool)
        .await;
    match result {
        Ok(creator) => (
            StatusCode::OK,
            Json(ApiResponse::success(creator)),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(e.to_string())),
        ),
    }
}

// CHECK username availability
pub async fn check_username(
    Extension(pool): Extension<PgPool>,
    Path(username): Path<String>,
) -> (StatusCode, Json<ApiResponse<serde_json::Value>>) {
    let exists: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM creators WHERE username = $1")
        .bind(&username)
        .fetch_one(&pool)
        .await
        .unwrap_or(0);
    let available = exists == 0;
    (
        StatusCode::OK,
        Json(ApiResponse::success(json!({ "available": available })))
    )
}

#[derive(Deserialize)]
pub struct UpdateCreatorInput {
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub bio: Option<String>,
    pub profile_image: Option<String>,
    pub wallet_address: Option<String>,
}

pub async fn update_creator(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateCreatorInput>,
) -> (StatusCode, Json<ApiResponse<serde_json::Value>>) {
    let result = sqlx::query!(
        "UPDATE creators SET
        display_name = COALESCE($1, display_name),
        email = COALESCE($2, email),
        bio = COALESCE($3, bio),
        profile_image = COALESCE($4, profile_image),
        wallet_address = COALESCE($5, wallet_address)
        WHERE id = $6",
        payload.display_name,
        payload.email,
        payload.bio,
        payload.profile_image,
        payload.wallet_address,
        id
    )
    .execute(&pool)
    .await;
    match result {
        Ok(_) => (
            StatusCode::OK,
            Json(ApiResponse::success(json!({ "updated": true }))),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(e.to_string())),
        ),
    }
}

// DELETE creator
pub async fn delete_creator(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
) -> (StatusCode, Json<ApiResponse<serde_json::Value>>) {
    let result = sqlx::query!("DELETE FROM creators WHERE id = $1", id).
        execute(&pool).await;
    match result {
        Ok(_) => (
            StatusCode::OK,
            Json(ApiResponse::success(json!({ "deleted": true }))),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(e.to_string())),
        ),
    }
}