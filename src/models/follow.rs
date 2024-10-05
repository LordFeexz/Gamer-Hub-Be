use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct AdminLog {
    pub id: i32,
    pub follower_id: String,
    pub followed_id: String,
    pub created_at: String,
    pub updated_at: String,
}
