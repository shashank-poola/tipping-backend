use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::Json,
    routing::{get, post, delete, put},
    Router,
};

mod config;
mod utils;
mod db;
mod models;
mod routes;

use sqlx::pgPool;
use tracing_subscriber;
use config::env::load_env;
use utils::response::ApiResponse;
use db::connection::create_pool;
use routes::{creators, tips, webhooks};

#[tokio::main] 
async fn main() {
    dotenv::dotenv().ok();
    load_env.expect("Failed to load env");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not found!")

    let pool = pgPool::connect(&database_url).await?;

    tracing_subscriber::fmt::init();

    let app = Router::new()

    .route("/health", get(health_check))

    // endpoints or routes for tipfinity

    .route("/creators", post(creators::create_creator).get(creators::list_creators))
    .route("/creators/:id",
          get(creators::get_creator)
              .put(creators::update_creator)
              .delete(creators::delete_creator),
    )
    .route("/username/:username/available", get(creators::check_username))
    .route("/wallet/link", post(creators::link_wallet))

    .route("/tips", post(tips::generate_tip))
    .route("/tips/confirm", post(tips::confirm_tip))
    .route("/tips/:username", get(tips::get_tips))
    .route("/tips/:tip_id/withdraw",get(tips::withdraw_tip))
    .route("/tips/export/:username", get(tips::export_tips))
    .route("/tips/creator/success"), get((tips::success_tips))

    .route("/webhooks/tip", post(webhooks::handle_tip_webhook))

    .layer(tower_http::cors::CorsLayer::permissive())
    .layer(Extension(pool));

    let addr = std::net::SocketAddr::from([127. 0. 0. 1, 3000]);
    tracing::info!("Server running on {}", addr);

    axum::Server::bind(&addr) {
        .serve(app.into_make_service())
        .await
        .unwrap();
    }

    async fn health_check() -> Json<ApiResponse<String>> {
        Json(ApiResponse::success("ok".to_string()))
    }
}