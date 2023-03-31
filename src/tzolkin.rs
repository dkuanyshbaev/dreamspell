// ---------------------------------------
// Dreamspell tzolkin core
// ---------------------------------------
// use crate::descriptions::description;
use crate::tables::*;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Seal {
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
#[serde(crate = "rocket::serde")]
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
    pub fn calc(seals: &Seals, parts: &[u32; 3]) -> Self {
        let kin = Self::kin(parts);
        let archetype = Self::archetype(kin);

        // ???
        let seal = &seals.0.get(archetype.0 as usize);
        let type_seal = &seals.0.get(archetype.1 as usize);

        Tzolkin {
            archetype_name: "Шаман-Полководец".to_string(),
            archetype_image: "".to_string(),
            archetype_description: "Рассудительный циник и здравый эгоист. Не весёлый и не грустный. Ты - самый трезвый человек на вечеринке, который развозит всех по домам. Потому что заботливый и не хмелеешь так, как остальные.".to_string(),
            portrait_name: "Шаман".to_string(),
            portrait_image: "".to_string(),
            portrait_description: "Путешественник по мирам духов. Обладаешь осознанным рептильным мозгом - способностью предельно хладнокровно и прагматично с точки зрения того, что мы все - отдельные живые организмы и задача каждого из нас - выжить.
Всегда отмечаешь и ценишь альтруизм со стороны других и сам умеешь заботиться. Не хмелеешь в общепринятом понимании. Часть тебя всегда трезва и оценивает обстановку.
Тело диктует и руководит. Важно слушаться своих инстинктов. Сексуальное притяжение - повод для знакомства, неприятный запах от человека-повод не общаться с ним.
Негативное проявление печати: Подверженностью влиянию эмоций, истеричность. Зацикленность на физическом теле и ощущениях. Саморазрушительная жертвенность".to_string(),
type_name: "Полководец".to_string(),
type_image: "".to_string(),
type_description: "Ты ведешь себя, как Полководец: Ни веселый, ни грустный, ни добрый ни злой. Загадочный и томный, ведомый инстинктом, обанянием и осязанием. Прагматичный альтруист. который вдохновлен самоотверженной заботой о других и видит в этом заботу о себе.
При принятии решений важно учитывать, где чьи интересы и не действовать себе во вред, но и понимать о пользе общего блага.".to_string(),
        }
    }

    fn kin(parts: &[u32; 3]) -> u32 {
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

    fn archetype(kin: u32) -> (u32, u32) {
        ARCHETYPE_TABLE[kin as usize]
    }
}
