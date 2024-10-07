use crate::helpers::global::convert_primitive_to_naive;
use crate::models::game::Game;
use sqlx::PgPool;

pub async fn find_all_games(executor: &PgPool) -> Vec<Game> {
    let records = sqlx::query!(
        r#"
        SELECT id, name, code, image_url, image_id, min_player, created_at, updated_at FROM games
        "#,
    )
    .fetch_all(executor)
    .await
    .unwrap_or_else(|_| Vec::new());

    let games: Vec<Game> = records
        .into_iter()
        .map(|record| Game {
            id: record.id,
            name: record.name,
            code: record.code,
            image_url: record.image_url,
            image_id: record.image_id,
            min_player: record.min_player,
            created_at: convert_primitive_to_naive(record.created_at),
            updated_at: convert_primitive_to_naive(record.updated_at),
        })
        .collect();

    games
}
