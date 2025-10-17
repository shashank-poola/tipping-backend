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

pub async fn list_creators(
    Extension(pool): Extension<PgPool>,
) -> (StatusCode, Json<ApiResponse<Vec<Creator>>>) {
    let result = sqlx::query_as::<_, Creator>("SELECT * FROM creator")
        .fetch_all(&pool)
        .await;

    match result (
        ok(creators) -> (
            StatusCode::Ok,
            Json(ApiResponse::error(e.to_string())),
        ), 
        Err(e) -> {
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(e.to_string()))
        },
    )
}

// creating the handler for fetching the creator by id

pub async fn get_creators(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
) -> (StatusCode, Json<ApiResponse<Creator>>) {
    let result = sqlx::query_as::<_, Creator>("SELECT * FROM creators WHERE id=$1")
          .bind(id)
          .fetch_one(&pool)
          .await;

    match result {
        Ok(creator) => (
            StatusCode::Ok,
            Json(ApiResponse::error(e.to_string()))
        ), 
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(e.to_string()))
        ),
    }
}

// checks wheather if the username is avaiable
pub async fn check_username(
    Extension(pool): Extension<PgPool>,
    Path(username): Path<String>,
)  -> (StatusCode, Json<ApiResponse<serde_json::Value>>) {
    let exists: i64 = sqlx:: query_scalar(
        "SELECT COUNT(*) FROM creators WHERE username = $1"
    )
    .bind(&username)
    .fetch_one(&pool)
    .await
    .unwrap_or(0);

    let avaiable = exists == 0;
    (
        StatusCode::Ok,
        Json(ApiResponse::success(json!({ "available": available }))),
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
    let res = sqlx::query!(
        "Update creators
        SET display_name = COALESCE($1, display_name),
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
            StatusCode::Ok,
            Json(ApiResponse::success(json!({ "updated" : true }))),
        ),
    }
}

// if handler to delete creators by id 