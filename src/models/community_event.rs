use serde::{Deserialize, Serialize};
use sqlx::FromRow;

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
    pub start_time: String,
    pub end_time: Option<String>,
    pub created_by: String,
    pub is_public: bool,
    pub status: CommunityEventStatus,
    pub created_at: String,
    pub updated_at: String,
}
