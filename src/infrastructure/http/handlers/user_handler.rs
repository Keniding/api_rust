use crate::domain::entities::user::User;
use crate::domain::services::user_service::UserService;
use crate::infrastructure::http::dto::user_dto::{CreateUserDto, PaginationDto};
use crate::shared::errors::AppError;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json
};
use std::sync::Arc;

pub async fn list_users(
    State(service): State<Arc<UserService>>,
    Query(pagination): Query<PaginationDto>,
) -> Result<Json<Vec<User>>, AppError> {
    let page = pagination.page.unwrap_or(1);
    let limit = pagination.limit.unwrap_or(10);
    let users = service.list_users(page, limit).await;
    Ok(Json(users))
}

pub async fn get_user(
    State(service): State<Arc<UserService>>,
    Path(id): Path<u64>,
) -> Result<Json<User>, AppError> {
    service
        .get_user(id)
        .await
        .map(Json)
        .ok_or(AppError::NotFound)
}

pub async fn create_user(
    State(service): State<Arc<UserService>>,
    Json(payload): Json<CreateUserDto>,
) -> Result<(StatusCode, Json<User>), AppError> {
    let user = service
        .create_user(payload.name, payload.email)
        .await
        .map_err(|e| AppError::BadRequest(e))?;

    Ok((StatusCode::CREATED, Json(user)))
}

pub async fn delete_user(
    State(service): State<Arc<UserService>>,
    Path(id): Path<u64>
) -> Result<StatusCode, AppError> {
    if service.delete_user(id).await {
        Ok(StatusCode::NO_CONTENT)
    } else { 
        Err(AppError::NotFound)
    }
}
