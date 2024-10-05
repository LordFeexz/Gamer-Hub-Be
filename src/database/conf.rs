use sqlx::{PgPool, Pool, Postgres};
use std::env;

pub async fn connect_to_db() -> Pool<Postgres> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    println!("Connected to database");

    pool
}
