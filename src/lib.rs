use chrono::{Datelike, Duration, NaiveDate, Weekday};
use computus;
use std::collections::BTreeMap;
use std::fmt::Display;

mod europe {
    use crate::{date, relative_to_easter_sunday, Holiday, HolidayRegion};
    use chrono::{Datelike, Duration, NaiveDate};

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum GermanHolidays {
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

    impl Holiday for GermanHolidays {
        fn to_date(&self, year: i32) -> Option<NaiveDate> {
            match self {
                GermanHolidays::Neujahr => date(year, 1, 1),
                GermanHolidays::HeiligeDreiKoenige => date(year, 1, 6),
                GermanHolidays::Frauentag => date(year, 1, 8),
                GermanHolidays::Karfreitag => relative_to_easter_sunday(year, -2),
                GermanHolidays::Ostermontag => relative_to_easter_sunday(year, 1),
                GermanHolidays::ErsterMai => date(year, 5, 1),
                GermanHolidays::ChristiHimmelfahrt => relative_to_easter_sunday(year, 39),
                GermanHolidays::Pfingstmontag => relative_to_easter_sunday(year, 50),
                GermanHolidays::Fronleichnam => relative_to_easter_sunday(year, 60),
                GermanHolidays::AugsburgerFriedensfest => date(year, 8, 8),
                GermanHolidays::MariaeHimmelfahrt => date(year, 8, 15),
                GermanHolidays::Weltkindertag => date(year, 9, 20),
                GermanHolidays::TagDerDeutschenEinheit => date(year, 10, 3),
                GermanHolidays::Reformationstag => date(year, 10, 31),
                GermanHolidays::Allerheiligen => date(year, 11, 1),
                GermanHolidays::BussUndBettag => bus_und_bettag(year),
                GermanHolidays::ErsterWeihnachtsfeiertag => date(year, 12, 25),
                GermanHolidays::ZweiterWeihnachtsfeiertag => date(year, 12, 26),
            }
        }
        fn description(&self) -> &'static str {
            match self {
                GermanHolidays::Neujahr => "Neujahr",
                GermanHolidays::HeiligeDreiKoenige => "Heilige Drei Könige",
                GermanHolidays::Frauentag => "Frauentag",
                GermanHolidays::Karfreitag => "Karfreitag",
                GermanHolidays::Ostermontag => "Ostermontag",
                GermanHolidays::ErsterMai => "Erster Mai",
                GermanHolidays::ChristiHimmelfahrt => "Christi Himmelfahrt",
                GermanHolidays::Pfingstmontag => "Pfingstmontag",
                GermanHolidays::Fronleichnam => "Fronleichnam",
                GermanHolidays::AugsburgerFriedensfest => "Augsburger Friedensfest",
                GermanHolidays::MariaeHimmelfahrt => "Mariä Himmelfahrt",
                GermanHolidays::Weltkindertag => "Weltkindertag",
                GermanHolidays::TagDerDeutschenEinheit => "Tag der Deutschen Einheit",
                GermanHolidays::Reformationstag => "Reformationstag",
                GermanHolidays::Allerheiligen => "Allerheiligen",
                GermanHolidays::BussUndBettag => "Buß- und Bettag",
                GermanHolidays::ErsterWeihnachtsfeiertag => "1. Weihnachtsfeiertag",
                GermanHolidays::ZweiterWeihnachtsfeiertag => "2. Weihnachtsfeiertag",
            }
        }
    }

    const BUNDESWEITE_FEIERTAGE: &'static [GermanHolidays] = &[
        GermanHolidays::Neujahr,
        GermanHolidays::Karfreitag,
        GermanHolidays::Ostermontag,
        GermanHolidays::ErsterMai,
        GermanHolidays::ChristiHimmelfahrt,
        GermanHolidays::Pfingstmontag,
        GermanHolidays::TagDerDeutschenEinheit,
        GermanHolidays::ErsterWeihnachtsfeiertag,
        GermanHolidays::ZweiterWeihnachtsfeiertag,
    ];

    enum Germany {
        BadenWuerttemberg,
        Bayern,
        Berlin,
        Brandenburg,
        Bremen,
        Hamburg,
        Hessen,
        MechlenburgVorpommern,
        Niedersachsen,
        NordrheinWestfalen,
        RheinlandPfalz,
        Saarland,
        Sachsen,
        SachsenAnhalt,
        SchleswigHolstein,
        Thueringen,
    }

    impl Germany {
        fn region_specific_holidays(&self) -> &'static [GermanHolidays] {
            match self {
                Germany::BadenWuerttemberg => &[
                    GermanHolidays::HeiligeDreiKoenige,
                    GermanHolidays::Fronleichnam,
                    GermanHolidays::Allerheiligen,
                ],
                Germany::Bayern => &[
                    GermanHolidays::HeiligeDreiKoenige,
                    GermanHolidays::Fronleichnam,
                    GermanHolidays::MariaeHimmelfahrt,
                    GermanHolidays::Allerheiligen,
                ],
                Germany::Berlin => &[GermanHolidays::Frauentag],
                Germany::Brandenburg => &[GermanHolidays::Reformationstag],
                Germany::Bremen => &[GermanHolidays::Reformationstag],
                Germany::Hamburg => &[GermanHolidays::Reformationstag],
                Germany::Hessen => &[GermanHolidays::Fronleichnam],
                Germany::MechlenburgVorpommern => &[GermanHolidays::Reformationstag],
                Germany::Niedersachsen => &[GermanHolidays::Reformationstag],
                Germany::NordrheinWestfalen => {
                    &[GermanHolidays::Fronleichnam, GermanHolidays::Allerheiligen]
                }
                Germany::RheinlandPfalz => {
                    &[GermanHolidays::Fronleichnam, GermanHolidays::Allerheiligen]
                }
                Germany::Saarland => &[
                    GermanHolidays::Fronleichnam,
                    GermanHolidays::MariaeHimmelfahrt,
                    GermanHolidays::Allerheiligen,
                ],
                Germany::Sachsen => &[
                    GermanHolidays::Reformationstag,
                    GermanHolidays::BussUndBettag,
                ],
                Germany::SachsenAnhalt => &[
                    GermanHolidays::HeiligeDreiKoenige,
                    GermanHolidays::Reformationstag,
                ],
                Germany::SchleswigHolstein => &[GermanHolidays::Reformationstag],
                Germany::Thueringen => &[
                    GermanHolidays::Weltkindertag,
                    GermanHolidays::Reformationstag,
                ],
            }
        }
    }

    impl HolidayRegion<GermanHolidays> for Germany {
        fn holidays(&self) -> Vec<GermanHolidays> {
            BUNDESWEITE_FEIERTAGE
                .iter()
                .cloned()
                .chain(self.region_specific_holidays().iter().cloned())
                .collect()
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

    #[cfg(test)]
    mod tests {

        use crate::europe::GermanHolidays::*;
        use crate::europe::Germany::*;
        use crate::europe::{bus_und_bettag, Germany};
        use crate::{date, HolidayRegion, ToHoliday};
        use chrono::{Datelike, NaiveDate, Weekday};
        use proptest::prelude::*;

        #[test]
        fn neujahr_feiertag_in_bayern() {
            let date = NaiveDate::from_ymd(2018, 01, 01);
            assert!(date.is_holiday(Bayern));
            assert_eq!(Some(Neujahr), date.holiday(Bayern));
        }

        #[test]
        fn total_number_holidays() {
            let number_holidays = |region: Germany| region.holidays_in_year(2019).len();
            assert_eq!(12, number_holidays(BadenWuerttemberg));
            assert_eq!(13, number_holidays(Bayern));
            assert_eq!(10, number_holidays(Berlin));
            assert_eq!(10, number_holidays(Brandenburg));
            assert_eq!(10, number_holidays(Bremen));
            assert_eq!(10, number_holidays(Hamburg));
            assert_eq!(10, number_holidays(Hessen));
            assert_eq!(10, number_holidays(MechlenburgVorpommern));
            assert_eq!(10, number_holidays(Niedersachsen));
            assert_eq!(11, number_holidays(NordrheinWestfalen));
            assert_eq!(11, number_holidays(RheinlandPfalz));
            assert_eq!(12, number_holidays(Saarland));
            assert_eq!(11, number_holidays(Sachsen));
            assert_eq!(11, number_holidays(SachsenAnhalt));
            assert_eq!(10, number_holidays(SchleswigHolstein));
            assert_eq!(11, number_holidays(Thueringen));
        }

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
    }

}

fn date(year: i32, month: u32, day: u32) -> Option<NaiveDate> {
    Some(NaiveDate::from_ymd(year, month, day))
}

fn relative_to_easter_sunday(year: i32, days_offset: i64) -> Option<NaiveDate> {
    let easter_sunday = computus::gregorian(year).ok()?;
    let date = NaiveDate::from_ymd_opt(easter_sunday.year, easter_sunday.month, easter_sunday.day)?;
    Some(date + Duration::days(days_offset))
}

trait Holiday {
    fn to_date(&self, year: i32) -> Option<NaiveDate>;
    fn description(&self) -> &'static str;
}

trait HolidayRegion<H>
where
    H: Holiday,
{
    fn holidays(&self) -> Vec<H>;

    fn is_holiday(&self, date: NaiveDate) -> bool {
        self.holiday_from_date(date).is_some()
    }
    fn holiday_from_date(&self, date: NaiveDate) -> Option<H> {
        self.holidays()
            .into_iter()
            .find(|holiday| holiday.to_date(date.year()) == Some(date))
    }
    fn holidays_in_year(&self, year: i32) -> Vec<(NaiveDate, H)> {
        let mut holidays_with_date: Vec<(NaiveDate, H)> = self
            .holidays()
            .into_iter()
            .map(|holiday| (holiday.to_date(year), holiday))
            .filter(|(date, _)| date.is_some())
            .map(|(date, holiday)| (date.unwrap(), holiday))
            .collect();
        holidays_with_date.sort_by_key(|(date, _)| *date);
        holidays_with_date
    }
}

trait ToHoliday<R, H>
where
    H: Holiday,
    R: HolidayRegion<H>,
{
    fn is_holiday(&self, region: R) -> bool;
    fn holiday(&self, region: R) -> Option<H>;
}

impl<R, H> ToHoliday<R, H> for NaiveDate
where
    H: Holiday,
    R: HolidayRegion<H>,
{
    fn is_holiday(&self, region: R) -> bool {
        region.is_holiday(*self)
    }
    fn holiday(&self, region: R) -> Option<H> {
        region.holiday_from_date(*self)
    }
}
