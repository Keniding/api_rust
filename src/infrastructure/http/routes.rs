use crate::domain::services::user_service::UserService;
use crate::infrastructure::http::handlers::{health_handler, user_handler};
use axum::{
    routing::get,
    Router
};
use std::sync::Arc;

pub fn create_routes(user_service: Arc<UserService>) -> Router {
    Router::new()
        .route("/health", get(health_handler::health_check))
        .route("/api/users", get(user_handler::list_users).post(user_handler::create_user))
        .route("/api/users/{id}", get(user_handler::get_user).delete(user_handler::delete_user))
        .with_state(user_service)
}