use chrono::{Datelike, Duration, NaiveDate};
use computus;

/// All reoccurring holidays in Germany.
/// This list contains both public and non-public holidays.
///
/// For public holidays use `GermanRegion` instead, since
/// public holidays differ from region to region.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GermanHoliday {
    Neujahr,
    HeiligeDreiKoenige,
    Frauentag,
    Faschingsdienstag,
    Aschermittwoch,
    Gruendonnerstag,
    Karfreitag,
    Ostersonntag,
    Ostermontag,
    ErsterMai,
    ChristiHimmelfahrt,
    Pfingstsonntag,
    Pfingstmontag,
    Fronleichnam,
    AugsburgerFriedensfest,
    MariaeHimmelfahrt,
    Weltkindertag,
    TagDerDeutschenEinheit,
    Reformationstag,
    Allerheiligen,
    BussUndBettag,
    Heiligabend,
    ErsterWeihnachtsfeiertag,
    ZweiterWeihnachtsfeiertag,
    Silvester,
}

use GermanHoliday::*;

impl GermanHoliday {
    /// Calculates the date for a specific year.
    ///
    /// `None` if it cannot be calculated.
    pub fn date(&self, year: i32) -> Option<NaiveDate> {
        match self {
            Neujahr => date(year, 1, 1),
            HeiligeDreiKoenige => date(year, 1, 6),
            Frauentag => date(year, 3, 8),
            Faschingsdienstag => relative_to_easter_sunday(year, -47),
            Aschermittwoch => relative_to_easter_sunday(year, -46),
            Gruendonnerstag => relative_to_easter_sunday(year, -3),
            Karfreitag => relative_to_easter_sunday(year, -2),
            Ostersonntag => relative_to_easter_sunday(year, 0),
            Ostermontag => relative_to_easter_sunday(year, 1),
            ErsterMai => date(year, 5, 1),
            ChristiHimmelfahrt => relative_to_easter_sunday(year, 39),
            Pfingstsonntag => relative_to_easter_sunday(year, 49),
            Pfingstmontag => relative_to_easter_sunday(year, 50),
            Fronleichnam => relative_to_easter_sunday(year, 60),
            AugsburgerFriedensfest => date(year, 8, 8),
            MariaeHimmelfahrt => date(year, 8, 15),
            Weltkindertag => date(year, 9, 20),
            TagDerDeutschenEinheit => date(year, 10, 3),
            Reformationstag => date(year, 10, 31),
            Allerheiligen => date(year, 11, 1),
            BussUndBettag => bus_und_bettag(year),
            Heiligabend => date(year, 12, 24),
            ErsterWeihnachtsfeiertag => date(year, 12, 25),
            ZweiterWeihnachtsfeiertag => date(year, 12, 26),
            Silvester => date(year, 12, 31),
        }
    }
    pub fn description(&self) -> &'static str {
        match self {
            Neujahr => "Neujahr",
            HeiligeDreiKoenige => "Heilige Drei Könige",
            Frauentag => "Frauentag",
            Faschingsdienstag => "Faschingsdienstag",
            Aschermittwoch => "Aschermittwoch",
            Gruendonnerstag => "Gründonnerstag",
            Karfreitag => "Karfreitag",
            Ostersonntag => "Ostersonntag",
            Ostermontag => "Ostermontag",
            ErsterMai => "Erster Mai",
            ChristiHimmelfahrt => "Christi Himmelfahrt",
            Pfingstsonntag => "Pfingstsonntag",
            Pfingstmontag => "Pfingstmontag",
            Fronleichnam => "Fronleichnam",
            AugsburgerFriedensfest => "Augsburger Friedensfest",
            MariaeHimmelfahrt => "Mariä Himmelfahrt",
            Weltkindertag => "Weltkindertag",
            TagDerDeutschenEinheit => "Tag der Deutschen Einheit",
            Reformationstag => "Reformationstag",
            Allerheiligen => "Allerheiligen",
            BussUndBettag => "Buß- und Bettag",
            Heiligabend => "Heiligabend",
            ErsterWeihnachtsfeiertag => "Erster Weihnachtsfeiertag",
            ZweiterWeihnachtsfeiertag => "Zweiter Weihnachtsfeiertag",
            Silvester => "Silvester",
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
    NaiveDate::from_ymd_opt(year, month, day)
}

fn relative_to_easter_sunday(year: i32, days_offset: i64) -> Option<NaiveDate> {
    let easter_sunday = computus::gregorian(year).ok()?;
    let date = NaiveDate::from_ymd_opt(easter_sunday.year, easter_sunday.month, easter_sunday.day)?;
    Some(date + Duration::days(days_offset))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DateExt;
    use chrono::Weekday;
    use proptest::prelude::*;

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

    macro_rules! holiday_tests {
    ($($name:ident: $holiday:expr, $date:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let holiday: GermanHoliday = $holiday;
            let (year, month, day) = $date;
            let date = NaiveDate::from_ymd(year, month, day);
            assert!(date.is_holiday(holiday));
        }
    )*
    }
}

    holiday_tests! {
        neujahr: Neujahr, (2019, 1, 1),
        dreikoenige: HeiligeDreiKoenige, (2019, 1, 6),
        frauentag: Frauentag, (2019, 3, 8),
        faschingdienstag: Faschingsdienstag, (2019, 3, 5),
        aschermittwoch: Aschermittwoch, (2019, 3, 6),
        gruendonnerstag: Gruendonnerstag, (2019, 4, 18),
        karfreitag: Karfreitag, (2019, 4, 19),

        ostersonntag1: Ostersonntag, (2016, 3, 27),
        ostersonntag2: Ostersonntag, (2017, 4, 16),
        ostersonntag3: Ostersonntag, (2018, 4, 1),
        ostersonntag4: Ostersonntag, (2019, 4, 21),
        ostersonntag5: Ostersonntag, (2020, 4, 12),

        ostermontag: Ostermontag, (2019, 4, 22),
        erstermai: ErsterMai, (2019, 5, 1),
        christi_himmelfahrt: ChristiHimmelfahrt, (2019, 5, 30),
        pfingstsonntag: Pfingstsonntag, (2019, 6, 9),
        pfingstmontag: Pfingstmontag, (2019, 6, 10),
        fronleichnam: Fronleichnam, (2019, 6, 20),
        augsburger_friedensfest: AugsburgerFriedensfest, (2019, 8, 8),
        mariae_himmelfahrt: MariaeHimmelfahrt, (2019, 8, 15),
        weltkindertag: Weltkindertag, (2019, 9, 20),
        deutsche_einheit: TagDerDeutschenEinheit, (2019, 10, 3),
        reformationstag: Reformationstag, (2019, 10, 31),
        allerheiligen: Allerheiligen, (2019, 11, 1),

        bus_und_bettag1: BussUndBettag, (2018, 11, 21),
        bus_und_bettag2: BussUndBettag, (2019, 11, 20),
        bus_und_bettag3: BussUndBettag, (2020, 11, 18),
        bus_und_bettag4: BussUndBettag, (2021, 11, 17),
        bus_und_bettag5: BussUndBettag, (2022, 11, 16),
        bus_und_bettag6: BussUndBettag, (2023, 11, 22),

        heiligabend: Heiligabend, (2019, 12, 24),
        erster_weihnachtsfeiertag: ErsterWeihnachtsfeiertag, (2019, 12, 25),
        zweiter_weihnachtsfeiertag: ZweiterWeihnachtsfeiertag, (2019, 12, 26),
        silvester: Silvester, (2019, 12, 31),
    }

}
