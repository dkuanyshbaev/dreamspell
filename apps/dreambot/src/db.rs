//////////////////////////////////////////
// Dreambot db
//////////////////////////////////////////
use sqlx::SqlitePool;

pub async fn save_birthday(db_pool: &SqlitePool, id: i64, birthday: String) {
    if let Err(e) = sqlx::query(
        "INSERT INTO users (id, birthday) VALUES ($1, $2)
        ON CONFLICT (id) DO UPDATE SET birthday=excluded.birthday",
    )
    .bind(id)
    .bind(&birthday)
    .execute(db_pool)
    .await
    {
        log::error!(
            "Failed to save birthday for user {}: {} (birthday: {})", 
            id, e, birthday
        );
    } else {
        log::debug!("Successfully saved birthday for user {}: {}", id, birthday);
    }
}
