use axum::{
    http::StatusCode, 
    response::IntoResponse, 
    routing::get, Router
};
use dotenv::dotenv;
use std::env;
use tracing_subscriber::{prelude::*};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let host = env::var("HOST").expect("");

    // init trace
    tracing_subscriber::registry()
    .with(
        tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "debug".into()),
    )
    .with(tracing_subscriber::fmt::layer())
    .init();

    let app = create_app().await;
    let listener = tokio::net::TcpListener::bind(&host).await.unwrap();

    tracing::info!("listening on {}", host);
    axum::serve(listener, app).await.unwrap();
}

#[tracing::instrument]
async fn create_app() -> Router {
    tracing::info!("create router...");
    Router::new()
        .route("/api/v1", get(health))
}

#[tracing::instrument]
async fn health() -> impl IntoResponse {
    tracing::info!("access health check endpoint");
    (StatusCode::OK, "healthcheck")
}
