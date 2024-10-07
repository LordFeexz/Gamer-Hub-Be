use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct ReplyComment {
    pub id: i32,
    pub user_id: Uuid,
    pub text: String,
    pub post_id: i32,
    pub comment_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub search_vector: String,
    pub trgm_similarity: f32,
}
