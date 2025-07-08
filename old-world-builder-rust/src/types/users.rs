use chrono::Utc;
use serde::{Deserialize, Serialize};
use bcrypt::{hash, verify, DEFAULT_COST}; 

// sqlx::FromRow
#[derive(Debug, Serialize)]
pub struct user {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub created_at: chrono::DateTime<Utc>, 
    pub updated_at: chrono::DateTime<Utc>,
    pub active: bool,
    pub deleted_at: Option<chrono::DateTime<Utc>>,
}

impl user {
    pub fn new() -> Self {
        user{
            id: 0,
            first_name: String::from(""),
            last_name: String::from(""),
            email: String::from(""),
            password: String::from(""),
            created_at:  sqlx::types::chrono::Utc::now().to_utc(),
            updated_at: sqlx::types::chrono::Utc::now().to_utc(),
            active: true,
            deleted_at: None,
        }
    }

    pub fn set_password(&mut self, new_password: String) {
        self.password = hash(new_password, DEFAULT_COST).unwrap();
    }

    pub fn verify_password(&mut self, password: String) -> bool {
        verify(password, &self.password).unwrap()
    }
}

#[derive(Deserialize)]
pub struct create_user {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub password_confirm: String,
}

#[derive(Deserialize)]
pub struct update_user {
    pub first_name: String,
    pub last_name: String,
}