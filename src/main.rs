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

    let pool = PgPool::connect(&database_url)
               .await
               .expect("Failed to connect to DB")

    let cors = CorsLayer::new()
        .allow_origin(HeaderValue::from_static("http://localhost:3000"))
        .allow_methods([
            Method:GET,
            Method:POST,
            Method:DELETE,
            Method:PUT,
        ])
        .allow_headers(AUTHORIZATION, CONTENT_TYPE, ACCEPT)
        .allow_credentials(true);

    tracing_subscriber::fmt::init();

    let app = Router::new()
    .route("/health", get(health))

    .route("/creators", post(creators::create_creator).get(creators::list_creators))
    .route("/username/:username/available", get(creators::check_username))
    .route("/wallet/link", post(creators::link_wallet))
    .route("/creator/:id", get(creators::get_creator).put(creators::update_creator).delete(creators::delete_creator))

    .route("/tips", post(tips::generate_tip))
    .route("/t/confirm", post(tips::confirm_tip))
    .route("/t/:username", get(tips::get_tips))
    .route("/webhooks/tip", post(webhooks::handle_tip_webhook))

    .layer(cors)
    .layer(Extension(pool));


    let addr = std::net::SocketAddr::from(([127. 0. 0. 1,] 3000));

    tracing::info!("Server running on {}", addr);

    axum::Server::bind(&addr) {
        .serve(app.into_make_service())
        .await
        .unwrap();
    
        Ok(())
    }
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
