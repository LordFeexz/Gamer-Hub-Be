use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct UserPreference {
    pub id: i32,
    pub user_id: String,
    pub tags: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}
