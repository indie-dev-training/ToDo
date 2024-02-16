use crate::repositories::{
    TodoRepository,
    NewTodo
};

use axum::{
    extract::{
        Extension,
        Path
    },
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

pub async fn get_all_todo<T: TodoRepository>(
    Extension(repository): Extension<Arc<T>>
) -> impl IntoResponse {
    let todo = repository.find_all();
    (StatusCode::OK, Json(todo))
}

pub async fn get_todo<T: TodoRepository> (
    Extension(repository): Extension<Arc<T>>,
    Path(id): Path<u32>
) -> Result<impl IntoResponse, StatusCode> {
    let todo = repository.find(id).ok_or(StatusCode::NOT_FOUND)?;
    Ok((StatusCode::OK, Json(todo)))
}