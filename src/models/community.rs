use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct Community {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub image_id: Option<String>,
    pub owner: Uuid,
    pub is_discord_server: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub search_vector_name: Option<String>,
    pub search_vector_description: Option<String>,
    pub name_trgm_similarity: f32,
    pub description_trgm_similarity: f32,
}
