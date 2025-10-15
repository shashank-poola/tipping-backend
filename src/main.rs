use tracing_subscriber;
use std::net::SocketAddr;
use dotenv::dotenv;
use tokio::net::TcpListener;

mod routes;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenv().ok();

    let app = routes::routes::create_router();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000)); 
    println!("Server running on http://{}", addr);

    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
