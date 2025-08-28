//////////////////////////////////////////
// Tzolkin db
//////////////////////////////////////////
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Seal {
    pub id: u8,
    pub name: String,
    pub image: String,
    pub archetype: String,
    pub archetype_description: String,
    pub portrait_description: String,
    pub type_description: String,
}

pub async fn get_seal(db_pool: &SqlitePool, index: u32) -> Result<Seal, sqlx::Error> {
    sqlx::query_as::<_, Seal>("SELECT * FROM seals WHERE id = ?")
        .bind(index)
        .fetch_one(db_pool)
        .await
}

pub async fn get_all_seals(db_pool: &SqlitePool) -> Result<Vec<Seal>, sqlx::Error> {
    sqlx::query_as::<_, Seal>("SELECT * FROM seals ORDER BY id")
        .fetch_all(db_pool)
        .await
}

pub async fn update_seal(db_pool: &SqlitePool, seal: &Seal) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE seals SET name = ?, image = ?, archetype = ?, archetype_description = ?, portrait_description = ?, type_description = ? WHERE id = ?"
    )
    .bind(&seal.name)
    .bind(&seal.image)
    .bind(&seal.archetype)
    .bind(&seal.archetype_description)
    .bind(&seal.portrait_description)
    .bind(&seal.type_description)
    .bind(seal.id)
    .execute(db_pool)
    .await?;
    
    Ok(())
}
