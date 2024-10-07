use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
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
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    search_vector_username: String,
    search_vector_bio: String,
    trgm_similarity_username: f64,
    trgm_similarity_bio: f64,
    pub is_blocked: bool,
    pub blocked_by: Option<Uuid>,
    pub block_reason: Option<String>,
}
