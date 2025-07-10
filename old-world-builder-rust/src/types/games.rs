use chrono::Utc;
use serde::{Serialize};

#[derive(Debug, Serialize)]
pub struct Game {
    pub id: i64,
    pub name: String,
    pub created_at: chrono::DateTime<Utc>, 
    pub updated_at: chrono::DateTime<Utc>,
    pub active: bool,
    pub deleted_at: Option<chrono::DateTime<Utc>>,
}

impl Game {
    pub fn new() -> Self {
        Game{
            id: 0,
            name: String::from(""),
            created_at:  sqlx::types::chrono::Utc::now().to_utc(),
            updated_at: sqlx::types::chrono::Utc::now().to_utc(),
            active: true,
            deleted_at: None,
        }
    }
}