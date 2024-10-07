use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct SearchHistory {
    pub id: i32,
    pub user_id: Uuid,
    pub searched_text: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
