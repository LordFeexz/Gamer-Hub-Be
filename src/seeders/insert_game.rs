use std::fs;
use serde::{Serialize,Deserialize};
use crate::database::conf::connect_to_db;

#[derive(Deserialize, Serialize)]
struct GameSeeder {
    name: String,
    code:String,
    image_url:String,
    image_id:String,
    min_player:i16
}

#[allow(dead_code)]
async fn insert_game() -> Result<(), Box<dyn std::error::Error>> {
    let pool = connect_to_db().await;
    let datas = fs::File::open("data/games.json")?;

    let games:Vec<GameSeeder> = serde_json::from_reader(&datas)?;

    let mut transaction = pool.begin().await?;

    for game in games {
        sqlx::query!(
            "INSERT INTO games (name, code, image_url, image_id, min_player) VALUES ($1, $2, $3, $4, $5)",
            game.name,
            game.code,
            game.image_url,
            game.image_id,
            game.min_player as i32,
        )
        .execute(&mut *transaction) 
        .await?;
    }

    transaction.commit().await?;

    println!("success insert game datas");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_insert_game() {
        insert_game().await.unwrap();
    }
}