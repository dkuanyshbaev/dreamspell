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
