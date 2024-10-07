use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum MediaType {
    IMAGE,
    VIDEO,
}

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct PostMedia {
    pub id: i32,
    pub media_type: MediaType,
    pub url: String,
    pub file_id: String,
    pub post_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
