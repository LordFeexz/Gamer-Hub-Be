use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: String, // UUID
    pub username: String,
    pub email: String,
    pub password: String,
    pub is_verified: bool,
    pub bio: Option<String>,
    pub image_url: Option<String>,
    pub image_id: Option<String>,
    pub banner_image_url: Option<String>,
    pub banner_image_id: Option<String>,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
    search_vector_username: Option<String>,
    search_vector_bio: Option<String>,
    trgm_similarity_username: Option<f64>,
    trgm_similarity_bio: Option<f64>,
    pub is_blocked: bool,
    pub blocked_by: Option<String>,
    pub block_reason: Option<String>,
}
