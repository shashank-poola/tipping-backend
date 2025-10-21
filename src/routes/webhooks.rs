use axum::{Json, http::StatusCode};
use serde_json::Value;
use crate::utils::response::ApiResponse;

pub async fn handle_tip_webhook(Json(payload): Json<Value>) -> (StatusCode, Json<ApiResponse<Value>>) {
    // Currently this just returns success for any webhook
    // Expand this logic as needed
    (
        StatusCode::OK,
        Json(ApiResponse::success(payload)),
    )
}
