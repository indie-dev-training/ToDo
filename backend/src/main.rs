mod handlers;
mod repositories;

use crate::repositories::{
    TodoRepository,
    InMemoryTodoRepository
};

use crate::handlers::create_todo;

use axum::{
    extract::Extension,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post}, 
    Router
};
use dotenv::dotenv;
use std::{
    env,
    sync::Arc
};
use tracing_subscriber::prelude::*;

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

    let repository = InMemoryTodoRepository::new();
    let app = create_app(repository).await;
    let listener = tokio::net::TcpListener::bind(&host).await.unwrap();

    tracing::info!("listening on {}", host);
    axum::serve(listener, app).await.unwrap();
}

async fn create_app<T: TodoRepository>(repository: T) -> Router {
    tracing::info!("create router...");
    Router::new()
        .route("/api/v1", get(health))
        .route("/api/v1/todo", post(create_todo::<T>))
        .layer(Extension(Arc::new(repository)))
}

#[tracing::instrument]
async fn health() -> impl IntoResponse {
    tracing::info!("access health check endpoint");
    (StatusCode::OK, "healthcheck")
}
