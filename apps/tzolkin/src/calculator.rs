//////////////////////////////////////////
// Dreamspell tzolkin core
//////////////////////////////////////////
use crate::db;
use crate::tables::{ARCHETYPE_TABLE, MONTH_TABLE, YEAR_TABLE};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

// Constants for Tzolkin calendar calculations
const TZOLKIN_CYCLE_DAYS: u32 = 260;  // Total days in Tzolkin sacred calendar cycle
const YEAR_CYCLE_LENGTH: f32 = 52.0;  // Years in complete calendar cycle
const KIN_ARRAY_OFFSET: u32 = 1;      // Kin numbers start from 1, arrays from 0

#[derive(Debug, Clone, Copy)]
pub enum Language {
    Russian,
    English,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Tzolkin {
    pub archetype_name: String,
    pub archetype_image: String,
    pub archetype_description: String,
    pub portrait_name: String,
    pub portrait_image: String,
    pub portrait_description: String,
    pub type_name: String,
    pub type_image: String,
    pub type_description: String,
}

impl Tzolkin {
    pub async fn new(
        db_pool: &SqlitePool,
        lang: Language,
        parts: &[u32; 3]
    ) -> Self {
        // Handle empty date
        if parts.eq(&[0, 0, 0]) {
            return Self::empty_with_images();
        }

        // Calculate kin using shared logic
        let kin = kin_from_parts(parts);
        let (main_id, type_id) = archetype(kin);
        
        // Parallel database queries
        let (main_seal, type_seal) = match tokio::try_join!(
            db::get_seal(db_pool, main_id),
            db::get_seal(db_pool, type_id)
        ) {
            Ok(seals) => seals,
            Err(e) => {
                tracing::error!(error = %e, main_id, type_id, "Failed to fetch seals from database");
                return Self::empty_with_images();
            }
        };

        // Build compound name
        let compound_name = build_compound_name(&main_seal, &type_seal, lang);

        Self {
            archetype_name: compound_name,
            archetype_image: main_seal.image.clone(),
            archetype_description: main_seal.archetype_description.clone(),
            portrait_name: main_seal.archetype.clone(),
            portrait_image: main_seal.image.clone(),
            portrait_description: main_seal.portrait_description.clone(),
            type_name: type_seal.archetype.clone(),
            type_image: type_seal.image.clone(),
            type_description: type_seal.type_description.clone(),
        }
    }
    
    pub fn empty() -> Self {
        Self::default()
    }

    fn empty_with_images() -> Self {
        Self {
            archetype_image: "no_image.jpg".to_string(),
            portrait_image: "no_image.jpg".to_string(),
            type_image: "no_image.jpg".to_string(),
            ..Self::default()
        }
    }
}

// Functional API - Bot-compatible functions
pub fn kin(day: u32, month: u32, year: i32) -> u32 {
    let parts = [year as u32, month, day];
    kin_from_parts(&parts)
}

pub fn kin_from_parts(parts: &[u32; 3]) -> u32 {
    let (year, month, day) = (parts[0], parts[1], parts[2]);
    if day == 0 || month == 0 || year == 0 {
        return 0;
    }
    let year_index = year as f32 - ((year as f32 / YEAR_CYCLE_LENGTH).floor() * YEAR_CYCLE_LENGTH);
    let mut kin = day + MONTH_TABLE[month as usize - 1] + YEAR_TABLE[year_index as usize];
    if kin > TZOLKIN_CYCLE_DAYS {
        kin -= TZOLKIN_CYCLE_DAYS
    }
    kin
}

pub fn archetype(kin: u32) -> (u32, u32) {
    ARCHETYPE_TABLE[(kin - KIN_ARRAY_OFFSET) as usize]
}

// Separate function for building compound names
fn build_compound_name(main_seal: &db::Seal, type_seal: &db::Seal, lang: Language) -> String {
    if main_seal.name == type_seal.name {
        let prefix = match lang {
            Language::Russian => "Классический",
            Language::English => "Classic",
        };
        format!("{} {}", prefix, type_seal.name)
    } else {
        format!("{} - {}", main_seal.name, type_seal.name)
    }
}