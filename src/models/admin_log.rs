use crate::enums::global::HttpMethod;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct AdminLog {
    pub id: i32,
    pub admin_id: Uuid,
    pub path: String,
    pub status_code: i16,
    pub method: HttpMethod,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
