use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct AdminLog {
    pub id: String,
    pub image_url: Option<String>,
    pub banner_url: Option<String>,
    pub access_token: String,
    pub refresh_token: String,
    pub token_expires: i64,
    pub user_id: String,
    pub created_at: String,
    pub updated_at: String,
}
