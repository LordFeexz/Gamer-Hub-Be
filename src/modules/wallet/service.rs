use sqlx::{Error, Executor, Postgres};
use uuid::Uuid;

pub async fn create_wallet<'a, E>(executor: E, user_id: Uuid) -> Result<(), Error>
where
    E: Executor<'a, Database = Postgres>,
{
    sqlx::query!(
        "INSERT INTO wallets (id, user_id, balance, coin) VALUES ($1, $2, $3, $4)",
        Uuid::new_v4(),
        user_id,
        0.0 as f64,
        0.0 as f64
    )
    .execute(executor)
    .await?;

    Ok(())
}
