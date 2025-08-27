//////////////////////////////////////////
// Tzolkin errors
//////////////////////////////////////////
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TzolkinError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Invalid date: {0}")]
    InvalidDate(String),
    
    #[error("Seal not found: {0}")]
    SealNotFound(u32),
}