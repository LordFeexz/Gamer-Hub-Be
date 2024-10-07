use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct PostComment {
    pub id: i32,
    pub post_id: i32,
    pub user_id: Uuid,
    pub text: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub search_vector: String,
    pub trgm_similarity: f32,
}
