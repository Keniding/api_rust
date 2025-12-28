use crate::domain::entities::user::User;
use crate::domain::repositories::user_repository::UserRepository;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct InMemoryUserRepository {
    users: Arc<RwLock<Vec<User>>>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        Self {
            users: Arc::new(RwLock::new(vec![
                User::new(1, "Henry".to_string(), "kenidingh@gmail.com".to_string())
            ]))
        }
    }
}

#[async_trait]
impl UserRepository for InMemoryUserRepository {
    async fn find_all(&self, page: u32, limit: u32) -> Vec<User> {
        let users = self.users.read().await;
        let start = ((page - 1) * limit) as usize;
        let end = start + limit as usize;
        
        users.get(start..end.min(users.len()))
            .unwrap_or(&[])
            .to_vec()
    }

    async fn find_by_id(&self, id: u64) -> Option<User> {
        let users = self.users.read().await;
        users.iter().find(|user| user.id == id).cloned()
    }

    async fn create(&self, name: String, email: String) -> User {
        let mut users = self.users.write().await;
        let new_id = users.iter().map(| user | user.id).max().unwrap_or(0) + 1;
        
        let user = User::new(new_id, name, email);
        users.push(user.clone());
        user
    }

    async fn delete(&self, id: u64) -> bool {
        let mut users = self.users.write().await;
        if let Some (pos) = users.iter().position(|user| user.id == id){ 
            users.remove(pos);
            true
        } else { 
            false
        }
    }
}