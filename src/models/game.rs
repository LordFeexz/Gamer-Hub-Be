use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct Game {
    pub id: i32,
    pub name: String,
    pub code: String,
    pub image_url: String,
    pub image_id: String,
    pub created_at: String,
    pub updated_at: String,
    pub min_player: i16,
}
