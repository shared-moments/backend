pub mod views;
pub mod config;
pub mod db;
pub mod prisma;
pub mod services;
pub mod repositories;

use std::net::SocketAddr;

use dotenv::dotenv;
use tracing::info;

use crate::views::get_router;


#[tokio::main]
async fn main() {
    dotenv().ok();

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    let app = get_router().await;

    info!("Start webserver...");
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    info!("Webserver shutdown...")
}
