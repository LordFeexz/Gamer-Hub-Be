use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Privacy {
    Public,
    Private,
    FriendOnly,
}

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub user_id: String,
    pub text: Option<String>,
    pub allow_comment: bool,
    pub edited_text: bool,
    pub tags: Vec<String>,
    pub privacy: Privacy,
    pub count_like: i64,
    pub count_comment: i64,
    pub count_bookmark: i64,
    pub count_share: i64,
    pub created_at: String,
    pub updated_at: String,
    pub community_id: Option<i32>,
    pub search_vector: Option<String>,
    pub trgm_similarity: Option<f32>,
    pub is_blocked: bool,
    pub blocked_by: Option<String>,
    pub block_reason: Option<String>,
}
