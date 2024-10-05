use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct ProfileViewer {
    pub id: i32,
    pub target_id: String,
    pub viewer_id: String,
    pub created_at: String,
    pub updated_at: String,
}
