use axum::{
    http::StatusCode, 
    response::IntoResponse, 
    routing::get, Router
};
use dotenv::dotenv;
use std::env;


#[tokio::main]
async fn main() {
    dotenv().ok();
    let host = env::var("HOST").expect("");
    let app = create_app().await;
    let listener = tokio::net::TcpListener::bind(&host).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn create_app() -> Router {
    Router::new()
        .route("/api/v1", get(health))
}

async fn health() -> impl IntoResponse {
    (StatusCode::OK, "healthcheck")
}
