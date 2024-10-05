use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct UserAchievement {
    pub id: i32,
    pub achievement_id: i32,
    pub user_id: String,
    pub created_at: String,
    pub updated_at: String,
}
