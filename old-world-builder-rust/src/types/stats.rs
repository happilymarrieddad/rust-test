use chrono::Utc;
use serde::{Serialize};

#[derive(Debug, Serialize)]
pub struct Stat {
    pub id: i64,
    pub game_id: i64,
    pub display: String,
    pub position: i64,
    pub created_at: chrono::DateTime<Utc>, 
    pub updated_at: chrono::DateTime<Utc>,
    pub active: bool,
    pub deleted_at: Option<chrono::DateTime<Utc>>,
}

impl Stat {
    pub fn new() -> Self {
        Stat{
            id: 0,
            game_id: 0,
            display: String::from(""),
            position: 0,
            created_at:  sqlx::types::chrono::Utc::now().to_utc(),
            updated_at: sqlx::types::chrono::Utc::now().to_utc(),
            active: true,
            deleted_at: None,
        }
    }
}