use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct user {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub created_at: chrono::DateTime<chrono::Utc>, 
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl user {
    pub fn new() -> Self {
        user{
            id: 0,
            first_name: String::from(""),
            last_name: String::from(""),
            email: String::from(""),
            password: String::from(""),
            created_at: chrono::offset::Utc::now(),
            updated_at: chrono::offset::Utc::now(),
        }
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
    pub password: String,
    pub password_confirm: String,
}