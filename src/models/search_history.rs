use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct SearchHistory {
    pub id: i32,
    pub user_id: String,
    pub searched_text: String,
    pub created_at: String,
    pub updated_at: String,
}
