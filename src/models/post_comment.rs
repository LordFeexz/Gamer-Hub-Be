use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct PostComment {
    pub id: i32,
    pub post_id: i32,
    pub user_id: String,
    pub text: String,
    pub created_at: String,
    pub updated_at: String,
    pub search_vector: String,
    pub trgm_similarity: f32,
}
