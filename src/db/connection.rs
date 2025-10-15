use sqlx::{pgPool, postgres::PgPoolOptions};

pub async fn create_pool() -> Results<PgPool, sqlx::Error> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set")
    PgPoolOption::new()
        .max_connections(10)
        .connect(&database_url)
        .await()
}