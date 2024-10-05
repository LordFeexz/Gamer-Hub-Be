use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct Achievement {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub image_url: String,
    pub image_id: String,
    pub game_id: Option<i32>,
    pub community_id: Option<i32>,
    pub created_at: String,
    pub updated_at: String,
}
