use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct UserWallet {
    pub id: String,
    pub user_id: String,
    pub balance: f64,
    pub coin: f64,
    pub created_at: String,
    pub updated_at: String,
}
