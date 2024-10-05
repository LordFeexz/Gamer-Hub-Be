use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct Team {
    pub id: String,
    pub name: String,
    pub owner: String,
    pub image_url: Option<String>,
    pub image_id: Option<String>,
    pub game_id: i32,
    pub is_public: bool,
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub total_member: i32,
    pub max_member: i32,
}
