use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
}

impl User {
    pub fn new(id: u64, name: String, email: String) -> Self {
        Self { id, name, email }
    }
    
    pub fn validate_email(&self) -> Result<(), String> {
        if !self.email.contains('@') { 
            return Err("Invalid email format".to_string())
        }
        Ok(())
    }
}