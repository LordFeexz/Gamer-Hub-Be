use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum CommunityEventStatus {
    SCHEDULED,
    ONGOING,
    COMPLETED,
    CANCELLED,
}

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct CommunityEvent {
    pub id: i32,
    pub community_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub location: String,
    pub start_time: NaiveDateTime,
    pub end_time: Option<NaiveDateTime>,
    pub created_by: Uuid,
    pub is_public: bool,
    pub status: CommunityEventStatus,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
