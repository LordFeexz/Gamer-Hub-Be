use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct UserWallet {
    pub id: Uuid,
    pub user_id: Uuid,
    pub balance: f64,
    pub coin: f64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
