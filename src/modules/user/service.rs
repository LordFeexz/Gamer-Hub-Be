use super::structs::CreateUserProps;
use crate::models::user::UserStatus;
use sqlx::{Error, Executor};
use uuid::Uuid;

pub async fn find_unique<'a, E>(
    executor: E,
    username: &str,
    email: &str,
) -> Vec<(Uuid, String, String)>
where
    E: Executor<'a, Database = sqlx::Postgres>,
{
    let records = sqlx::query!(
        "SELECT id, username, email FROM users WHERE username = $1 OR email = $2",
        username,
        email
    )
    .fetch_all(executor)
    .await
    .unwrap_or_else(|_| Vec::new())
    .into_iter()
    .map(|r| (r.id, r.username, r.email))
    .collect();

    records
}

pub async fn create_one<'a, E>(executor: E, props: CreateUserProps) -> Result<Uuid, Error>
where
    E: Executor<'a, Database = sqlx::Postgres>,
{
    match sqlx::query!(
        "INSERT INTO users (id, username, email, password, is_verified, status) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id",
        Uuid::new_v4(),
        props.username,
        props.email,
        props.password,
        props.is_verified,
        match props.status  {
            UserStatus::ACTIVE => "active",
            UserStatus::INACTIVE => "inActive"
        },
    ).fetch_one(executor).await {
        Ok(record) => Ok(record.id),
        Err(err) => Err(err),
    }
}
