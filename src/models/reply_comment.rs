use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct ReplyComment {
    pub id: i32,
    pub user_id: String,
    pub text: String,
    pub post_id: i32,
    pub comment_id: i32,
    pub created_at: String,
    pub updated_at: String,
    pub search_vector: String,
    pub trgm_similarity: f32,
}
