use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CommunityMemberRole {
    ADMIN,
    MEMBER,
    OWNER,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CommunityMember {
    pub id: i32,
    pub user_id: Uuid,
    pub community_id: i32,
    pub role: CommunityMemberRole,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
