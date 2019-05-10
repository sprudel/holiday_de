use chrono::{Datelike, Duration, NaiveDate};
use computus;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GermanHoliday {
    Neujahr,
    HeiligeDreiKoenige,
    Frauentag,
    Karfreitag,
    Ostermontag,
    ErsterMai,
    ChristiHimmelfahrt,
    Pfingstmontag,
    Fronleichnam,
    AugsburgerFriedensfest,
    MariaeHimmelfahrt,
    Weltkindertag,
    TagDerDeutschenEinheit,
    Reformationstag,
    Allerheiligen,
    BussUndBettag,
    ErsterWeihnachtsfeiertag,
    ZweiterWeihnachtsfeiertag,
}

use GermanHoliday::*;

impl GermanHoliday {
    pub fn date(&self, year: i32) -> Option<NaiveDate> {
        match self {
            Neujahr => date(year, 1, 1),
            HeiligeDreiKoenige => date(year, 1, 6),
            Frauentag => date(year, 3, 8),
            Karfreitag => relative_to_easter_sunday(year, -2),
            Ostermontag => relative_to_easter_sunday(year, 1),
            ErsterMai => date(year, 5, 1),
            ChristiHimmelfahrt => relative_to_easter_sunday(year, 39),
            Pfingstmontag => relative_to_easter_sunday(year, 50),
            Fronleichnam => relative_to_easter_sunday(year, 60),
            AugsburgerFriedensfest => date(year, 8, 8),
            MariaeHimmelfahrt => date(year, 8, 15),
            Weltkindertag => date(year, 9, 20),
            TagDerDeutschenEinheit => date(year, 10, 3),
            Reformationstag => date(year, 10, 31),
            Allerheiligen => date(year, 11, 1),
            BussUndBettag => bus_und_bettag(year),
            ErsterWeihnachtsfeiertag => date(year, 12, 25),
            ZweiterWeihnachtsfeiertag => date(year, 12, 26),
        }
    }
    pub fn description(&self) -> &'static str {
        match self {
            Neujahr => "Neujahr",
            HeiligeDreiKoenige => "Heilige Drei Könige",
            Frauentag => "Frauentag",
            Karfreitag => "Karfreitag",
            Ostermontag => "Ostermontag",
            ErsterMai => "Erster Mai",
            ChristiHimmelfahrt => "Christi Himmelfahrt",
            Pfingstmontag => "Pfingstmontag",
            Fronleichnam => "Fronleichnam",
            AugsburgerFriedensfest => "Augsburger Friedensfest",
            MariaeHimmelfahrt => "Mariä Himmelfahrt",
            Weltkindertag => "Weltkindertag",
            TagDerDeutschenEinheit => "Tag der Deutschen Einheit",
            Reformationstag => "Reformationstag",
            Allerheiligen => "Allerheiligen",
            BussUndBettag => "Buß- und Bettag",
            ErsterWeihnachtsfeiertag => "1. Weihnachtsfeiertag",
            ZweiterWeihnachtsfeiertag => "2. Weihnachtsfeiertag",
        }
    }
}

fn bus_und_bettag(year: i32) -> Option<NaiveDate> {
    let reference_date = NaiveDate::from_ymd(year, 11, 23);
    let weekday_ordinal = i64::from(reference_date.weekday().num_days_from_monday());
    let duration_to_previous_wednesday = if weekday_ordinal < 3 {
        Duration::days(-(weekday_ordinal + 5))
    } else {
        Duration::days(2 - weekday_ordinal)
    };
    Some(reference_date + duration_to_previous_wednesday)
}

fn date(year: i32, month: u32, day: u32) -> Option<NaiveDate> {
    Some(NaiveDate::from_ymd(year, month, day))
}

fn relative_to_easter_sunday(year: i32, days_offset: i64) -> Option<NaiveDate> {
    let easter_sunday = computus::gregorian(year).ok()?;
    let date = NaiveDate::from_ymd_opt(easter_sunday.year, easter_sunday.month, easter_sunday.day)?;
    Some(date + Duration::days(days_offset))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Weekday;
    use proptest::prelude::*;

    #[test]
    fn test_bus_und_bettag_calc() {
        assert_eq!(date(2018, 11, 21), bus_und_bettag(2018));
        assert_eq!(date(2019, 11, 20), bus_und_bettag(2019));
        assert_eq!(date(2020, 11, 18), bus_und_bettag(2020));
        assert_eq!(date(2021, 11, 17), bus_und_bettag(2021));
        assert_eq!(date(2022, 11, 16), bus_und_bettag(2022));
        assert_eq!(date(2023, 11, 22), bus_und_bettag(2023));
    }

    proptest! {
    #[test]
    fn test_bus_und_bettag_is_wed_before_23th_nov(y in 1i32..2999) {
        let date = bus_und_bettag(y).unwrap();
        assert_eq!(Weekday::Wed, date.weekday());
        let duration = date.signed_duration_since(NaiveDate::from_ymd(y, 11, 23));
        assert!(duration.num_days() <= -1);
        assert!(duration.num_days() >= -7);
    }
    }

    proptest! {
    #[test]
    fn relative_to_easter_sunday_does_not_panic(year: i32, offset: i64) {
        relative_to_easter_sunday(year, offset);
    }
    }

}
