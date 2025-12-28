use axum::{
    extract::{Path, Query, State},
    http::{ StatusCode},
    response::Json,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct User {
    id: u64,
    name: String,
    email: String,
}

#[derive(Debug, Deserialize)]
struct CreateUser {
    name: String,
    email: String,
}

#[derive(Debug, Deserialize)]
struct Pagination {
    page: Option<u32>,
    limit: Option<u32>,
}

type AppState = Arc<RwLock<Vec<User>>>;

#[tokio::main]
async fn main() {
    let state = Arc::new(RwLock::new(vec![
        User {
            id: 1, name: "Henry".into(), email: "kendingh@gmail.com".into()
        },
    ]));

    let app = Router::new()
        .route("/api/users", get(list_users).post(create_user))
        .route("/api/user/{id}", get(get_user).delete(delete_user))
        .route("/health", get(health_check))
        .with_state(state)
        .layer(
            tower_http::cors::CorsLayer::permissive()
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("Listening on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn list_users(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
) -> Json<Vec<User>> {
    let users = state.read().await;
    let page = pagination.page.unwrap_or(1);
    let limit = pagination.limit.unwrap_or(10) as usize;

    let start = ((page - 1) * limit as u32) as usize;
    let end = start + limit;

    Json(users.get(start..end.min(users.len()))
        .unwrap_or(&[])
        .to_vec())
}

async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<User>, StatusCode> {
    let users = state.read().await;
    users.iter()
        .find(|u| u.id == id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    let mut users = state.write().await;
    let new_id = users.iter()
        .map(|u| u.id).max().unwrap_or(0) + 1;

    let user = User {
        id: new_id,
        name: payload.name,
        email: payload.email,
    };

    users.push(user.clone());
    (StatusCode::OK, Json(user))
}

async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> StatusCode {
    let mut users = state.write().await;
    if let Some(pos) = users.iter()
        .position(|u| u.id == id) {
        users.remove(pos);
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}
