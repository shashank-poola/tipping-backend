use std::env;
use axum::{
    extract::Extension,
    routing::{get, post, put, delete},
    response::Json,
    Router,
};
use hyper::header::{HeaderValue, AUTHORIZATION, CONTENT_TYPE, ACCEPT};
use hyper::Method;
use chrono::Utc;
use serde_json::json;
use sqlx::PgPool;
use tower_http::cors::CorsLayer;
use routes::{creators, tips, webhooks};

mod config;
mod utils;
mod db;
mod models;
mod routes {
    pub mod creators;
    pub mod tips;
}

#[tokio::main] 
async fn main() {
    dotenv::dotenv().ok();
    .expect("Failed to load env");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not found!");

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

    .route("/creators", post(create_creatorr).get(list_creators))
    .route("/username/:username/available", get(check_username))
    .route("/wallet/link", post(creators::link_wallet))
    .route("/creator/:id", get(get_creators).put(update_creator).delete(creators::delete_creator))

    .route("/tips", post(create_tip))
    .route("/t/confirm", post(tips::confirm_tip))
    .route("/t/:username", get(tips::get_tips))
    .route("/webhooks/tip", post(webhooks::handle_tip_webhook))

    .layer(cors)
    .layer(Extension<PgPool>);


    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::info!("Server running on {}", addr);

    axum::Server::bind(&addr) 
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
