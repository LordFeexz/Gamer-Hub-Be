use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

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
    pub user_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub role: TeamRole,
}
