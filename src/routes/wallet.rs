use axum::{
    extract::{Extension, Json},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;

use solana_sdk::{
    pubkey::Pubkey,
    signature::Signature,
};
use bs58;

use crate::utils::response::ApiResponse;

#[derive(Deserialize)]
pub struct WalletLinkInput {
    pub public_key: String,
    pub message: String,
    pub signature: String,
    pub creator_id: i32,
}

#[derive(Serialize)]
pub struct WalletLinkResponse {
    pub verified: bool,
    pub wallet_address: String,
}

pub async fn link_wallet(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<WalletLinkInput>,
) -> (StatusCode, Json<ApiResponse<serde_json::Value>>) {
    let public_key_bytes = match bs58::decode(&payload.public_key).into_vec() {
        Ok(bytes) => bytes,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::error("Invalid public key".to_string())),
            );
        }
    };
    let signature_bytes = match bs58::decode(&payload.signature).into_vec() {
        Ok(bytes) => bytes,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::error("Invalid signature".to_string())),
            );
        }
    };

    let pubkey = match Pubkey::try_from(public_key_bytes.as_slice()) {
        Ok(pk) => pk,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::error("Invalid public key bytes for Pubkey".to_string())),
            );
        }
    };

    let signature = match Signature::try_from(signature_bytes.as_slice()) {
        Ok(sig) => sig,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::error("Invalid signature bytes for Signature".to_string())),
            );
        }
    };

    let verified = signature.verify(pubkey.as_ref(), payload.message.as_bytes());
    if !verified {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::error("Signature verification failed".to_string())),
        );
    }

    let result = sqlx::query!(
        "UPDATE creators SET wallet_address = $1 WHERE id = $2",
        payload.public_key,
        payload.creator_id
    )
    .execute(&pool)
    .await;

    match result {
        Ok(_) => (
            StatusCode::OK,
            Json(ApiResponse::success(json!(WalletLinkResponse {
                verified: true,
                wallet_address: payload.public_key
            }))),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(e.to_string())),
        ),
    }
}