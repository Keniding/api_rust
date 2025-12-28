use crate::domain::entities::user::User;
use crate::domain::repositories::user_repository::UserRepository;
use std::sync::Arc;

pub struct UserService {
    repository: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        Self { repository }
    }

    pub async fn list_users(&self, page: u32, limit: u32) -> Vec<User> {
        self.repository.find_all(page, limit).await
    }

    pub async fn get_user(&self, id: u64) -> Option<User> {
        self.repository.find_by_id(id).await
    }

    pub async fn create_user(&self, name: String, email: String) -> Result<User, String> {
        if name.is_empty() {
            return Err("Name cannot be empty".to_string());
        }

        let user = self.repository.create(name, email).await;
        user.validate_email()?;
        Ok(user)
    }

    pub async fn delete_user(&self, id: u64) -> bool {
        self.repository.delete(id).await
    }
}