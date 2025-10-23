use axum::{
    extract::Extension,
    routing::{get, post},
    response::Json,
    Router,
};
use hyper::header::{HeaderValue, AUTHORIZATION, CONTENT_TYPE, ACCEPT};
use hyper::Method;
use chrono::Utc;
use serde_json::json;
use sqlx::PgPool;
use tower_http::cors::CorsLayer;

mod config;
mod utils;
mod db;
mod models;
pub mod routes {
    pub mod creators;
    pub mod tips;
    pub mod wallet;
    pub mod webhooks;
}
use routes::{creators, tips, wallet, webhooks};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not found!");

    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to DB");

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run database migrations");

    let cors = CorsLayer::new()
        .allow_origin(HeaderValue::from_static("http://localhost:3000"))
        .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PUT])
        .allow_headers([AUTHORIZATION, CONTENT_TYPE, ACCEPT])
        .allow_credentials(true);

    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/health", get(health))
        .route("/creators", post(creators::create_creator).get(creators::list_creators))
        .route("/username/:username/available", get(creators::check_username))
        .route("/wallet/link", post(wallet::link_wallet))
        .route("/creator/:id", get(creators::get_creator).put(creators::update_creator).delete(creators::delete_creator))
        .route("/tips", post(tips::create_tip))
        .route("/tips/creator/:creator_id", get(tips::get_tips_for_creator))
        .route("/tips/recent", get(tips::get_recent_tips))
        .route("/webhooks/tip", post(webhooks::handle_tip_webhook))
        .layer(cors)
        .layer(Extension(pool));

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::info!("Server running on {}", addr);

    tokio::net::windows::named_pipe::PipeEnd::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn health(Extension(pool): Extension<PgPool>) -> Json<serde_json::Value> {
    let res = sqlx::query_scalar::<_, i32>("SELECT 1")
        .fetch_one(&pool)
        .await;

    match res {
        Ok(v) => Json(json!({ "status": "ok", "db": v })),
        Err(e) => Json(json!({ "status": "error", "db_error": e.to_string() })),
    }
}
