use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CommunityMemberRole {
    ADMIN,
    MEMBER,
    OWNER,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CommunityMember {
    pub id: i32,
    pub user_id: String,
    pub community_id: i32,
    pub role: CommunityMemberRole,
    pub created_at: String,
    pub updated_at: String,
}
