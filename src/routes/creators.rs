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
mod models