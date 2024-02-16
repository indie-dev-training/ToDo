use crate::repositories::{
    TodoRepository,
    NewTodo
};

use axum::{
    extract::Extension,
    http::StatusCode,
    response::IntoResponse,
    Json
};

use std::sync::Arc;

pub async fn create_todo<T: TodoRepository> (
    Extension(repository): Extension<Arc<T>>,
    Json(payload): Json<NewTodo>
) -> impl IntoResponse {
    let todo = repository.create(payload);
    (StatusCode::CREATED, Json(todo))
}