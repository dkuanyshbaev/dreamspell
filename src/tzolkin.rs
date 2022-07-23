// ---------------------------------------
// Dreamspell tzolkin core
// ---------------------------------------
use crate::tables::*;

pub fn kin(parts: &[u32; 3]) -> u32 {
    let (day, month, year) = (parts[0], parts[1], parts[2]);

    if day == 0 || month == 0 || year == 0 {
        return 0;
    }

    let year_index = year as f32 - ((year as f32 / 52_f32).floor() * 52_f32);
    let mut kin = day + MONTH_TABLE[month as usize - 1] + YEAR_TABLE[year_index as usize];

    if kin > 260 {
        kin = kin - 260
    }
    kin
}

pub fn archetype(kin: u32) -> (u32, u32) {
    ARCHETYPE_TABLE[kin as usize]
}
