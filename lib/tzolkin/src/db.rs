//////////////////////////////////////////
// Tzolkin db
//////////////////////////////////////////
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Seal {
    pub id: u8,
    pub name: String,
    pub name_en: String,
    pub image: String,
    pub archetype: String,
    pub archetype_en: String,
    pub archetype_description: String,
    pub archetype_description_short: String,
    pub archetype_description_en: String,
    pub portrait_description: String,
    pub portrait_description_short: String,
    pub portrait_description_en: String,
    pub type_description: String,
    pub type_description_short: String,
    pub type_description_en: String,
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
        "UPDATE seals SET name = ?, name_en = ?, image = ?, archetype = ?, archetype_en = ?, archetype_description = ?, archetype_description_short = ?, archetype_description_en = ?, portrait_description = ?, portrait_description_short = ?, portrait_description_en = ?, type_description = ?, type_description_short = ?, type_description_en = ? WHERE id = ?"
    )
    .bind(&seal.name)
    .bind(&seal.name_en)
    .bind(&seal.image)
    .bind(&seal.archetype)
    .bind(&seal.archetype_en)
    .bind(&seal.archetype_description)
    .bind(&seal.archetype_description_short)
    .bind(&seal.archetype_description_en)
    .bind(&seal.portrait_description)
    .bind(&seal.portrait_description_short)
    .bind(&seal.portrait_description_en)
    .bind(&seal.type_description)
    .bind(&seal.type_description_short)
    .bind(&seal.type_description_en)
    .bind(seal.id)
    .execute(db_pool)
    .await?;
    
    Ok(())
}
