use crate::enums::global::HttpMethod;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct AdminLog {
    pub id: i32,
    pub admin_id: String,
    pub path: String,
    pub status_code: i16,
    pub method: HttpMethod,
    pub created_at: String,
    pub updated_at: String,
}
