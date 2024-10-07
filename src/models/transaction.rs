use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum TransactionType {
    DEPOSIT,
    WITHDRAW,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SupportedCurrency {
    USD,
    IDR,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum TransactionStatus {
    PENDING,
    COMPLETED,
    FAILED,
    CANCELLED,
    REFUND,
    SETTLEMENT,
    DENY,
    EXPIRED,
}

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Uuid,
    pub user_id: Uuid,
    pub amount: f64,
    pub transaction_type: TransactionType,
    pub currency: SupportedCurrency,
    pub status: TransactionStatus,
    pub description: Option<String>,
    pub detail: Option<String>,
    pub signature: String,
    pub discount: f64,
    pub fee: f64,
    pub tax: f64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
