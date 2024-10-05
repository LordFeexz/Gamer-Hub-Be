use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Division {
    DIRECTOR,
    FINANCE,
    IT,
    THIRDPARTY,
    CUSTOMERSERVICE,
    MARKETING,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Role {
    SUPERVISOR,
    MANAGER,
    STAFF,
}

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct Admin {
    pub id: String,
    pub fullname: String,
    pub email: String,
    pub password: String,
    pub division: Division,
    pub role: Role,
    pub created_at: String,
    pub updated_at: String,
}
