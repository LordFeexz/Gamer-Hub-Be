use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum TeamRole {
    OWNER,
    MEMBER,
    COACH,
    INSPECTOR,
    MANAGER,
    ADMIN,
}

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct TeamMember {
    pub id: i32,
    pub team_id: String,
    pub status: bool,
    pub user_id: String,
    pub created_at: String,
    pub updated_at: String,
    pub role: TeamRole,
}
