use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct Team {
    pub id: Uuid,
    pub name: String,
    pub owner: Uuid,
    pub image_url: Option<String>,
    pub image_id: Option<String>,
    pub game_id: i32,
    pub is_public: bool,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub total_member: i32,
    pub max_member: i32,
}
