use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum TournamentStatus {
    PREPARATION,
    ONGOING,
    FINISHED,
    CANCELLED,
}

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct Tournament {
    pub id: i32,
    pub name: String,
    pub game_id: i32,
    pub price_pool: f64,
    pub slot: i32,
    pub start_date: String,
    pub registration_fee: f64,
    pub description: Option<String>,
    pub status: TournamentStatus,
    pub image_url: String,
    pub image_id: String,
    pub location: String,
    pub tags: Vec<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub user_id: Option<Uuid>,
    pub community_id: Option<i32>,
    pub live_on: Option<String>,
    pub is_public: bool,
    pub money_pool: f64,
}
