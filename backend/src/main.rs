use axum::{
    http::StatusCode, 
    response::IntoResponse, 
    routing::get, Router
};


#[tokio::main]
async fn main() {
    let app = create_app().await;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn create_app() -> Router {
    Router::new()
        .route("/api/v1/", get(health))
}

async fn health() -> impl IntoResponse {
    (StatusCode::OK, "200 ok")
}
