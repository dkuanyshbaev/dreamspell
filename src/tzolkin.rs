// ---------------------------------------
// Dreamspell tzolkin core
// ---------------------------------------
use crate::tables::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Seal {
    id: u8,
    name: String,
    image: String,
    archetype: String,
    archetype_description: String,
    portrait_description: String,
    type_description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Seals(Vec<Seal>);

#[derive(Serialize, Deserialize)]
pub struct Tzolkin {
    archetype_name: String,
    archetype_image: String,
    archetype_description: String,
    portrait_name: String,
    portrait_image: String,
    portrait_description: String,
    type_name: String,
    type_image: String,
    type_description: String,
}

impl Tzolkin {
    pub fn new(seals: &Seals, parts: &[u32; 3]) -> Self {
        let kin = Self::kin(parts);
        let archetype = Self::archetype(kin);
        let main_seal = &seals.0.get((archetype.0 - 1) as usize);
        let type_seal = &seals.0.get((archetype.1 - 1) as usize);

        if archetype.0 == archetype.1 {
            // TODO: typical
            println!("typical");
        }

        println!("archetype: {:?}", archetype);

        if main_seal.is_none() || type_seal.is_none() {
            Self::empty()
        } else {
            let main_seal = main_seal.unwrap();
            let type_seal = type_seal.unwrap();

            println!(
                "main_seal: id: {:?}, name: {:?}",
                main_seal.id, main_seal.name
            );
            println!(
                "type_seal: id: {:?}, name: {:?}",
                type_seal.id, type_seal.name
            );

            Self {
                archetype_name: main_seal.archetype.to_owned(),
                archetype_image: main_seal.image.to_owned(),
                archetype_description: main_seal.archetype_description.to_owned(),
                portrait_name: main_seal.archetype.to_owned(),
                portrait_image: main_seal.image.to_owned(),
                portrait_description: main_seal.portrait_description.to_owned(),
                type_name: type_seal.archetype.to_owned(),
                type_image: type_seal.image.to_owned(),
                type_description: type_seal.type_description.to_owned(),
            }
        }
    }

    pub fn empty() -> Self {
        Self {
            archetype_name: "".to_string(),
            archetype_image: "".to_string(),
            archetype_description: "".to_string(),
            portrait_name: "".to_string(),
            portrait_image: "".to_string(),
            portrait_description: "".to_string(),
            type_name: "".to_string(),
            type_image: "".to_string(),
            type_description: "".to_string(),
        }
    }

    fn kin(parts: &[u32; 3]) -> u32 {
        let (year, month, day) = (parts[0], parts[1], parts[2]);
        if day == 0 || month == 0 || year == 0 {
            return 0;
        }
        let year_index = year as f32 - ((year as f32 / 52_f32).floor() * 52_f32);

        let mut kin = day + MONTH_TABLE[month as usize - 1] + YEAR_TABLE[year_index as usize];
        if kin > 260 {
            kin -= 260
        }

        println!("year: {:?}, month: {:?}, day:{:?}", year, month, day);
        println!("kin: {:?}", kin);

        kin
    }

    fn archetype(kin: u32) -> (u32, u32) {
        ARCHETYPE_TABLE[(kin - 1) as usize]
    }
}
