use crate::domain::entities::user::User;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_all(&self, page: u32, limit: u32) -> Vec<User>;
    async fn find_by_id(&self, id:u64) -> Option<User>;
    async fn create(&self, name: String, email: String) -> User;
    async fn delete(&self, id: u64) -> bool;
}