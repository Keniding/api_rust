use std::sync::Arc;
use tower_http::cors::CorsLayer;
use crate::config::settings::Settings;
use crate::domain::services::user_service::UserService;
use crate::infrastructure::http::routes::create_routes;
use crate::infrastructure::persistence::in_memory_user_repository::InMemoryUserRepository;

mod config;
mod domain;
mod infrastructure;
mod shared;

#[tokio::main]
async fn main() {
    let settings = Settings::new().expect("Failed to load settings");

    let user_repository = Arc::new(InMemoryUserRepository::new());
    let user_service = Arc::new(UserService::new(user_repository));

    let app = create_routes(user_service)
        .layer(CorsLayer::permissive());

    let addr = format!("{}:{}", settings.server.host, settings.server.port);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind TCP listener");

    print!("Server running on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}
