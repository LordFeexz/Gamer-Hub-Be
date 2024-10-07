use sqlx::{postgres::PgPoolOptions, Executor, Pool, Postgres};
use std::env;

pub async fn connect_to_db() -> Pool<Postgres> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    let _ = pool.execute("SET TIME ZONE 'Asia/Jakarta';").await;

    println!("Connected to database");

    pool
}
