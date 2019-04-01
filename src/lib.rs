use chrono::{Duration, Weekday};
use chrono::{Datelike, NaiveDate};
use computus;
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum GermanHolidays {
    Neujahr,
    HeiligeDreiKoenige,
    Frauentag,
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
    ErsterWeihnachtsfeiertag,
    ZweiterWeihnachtsfeiertag,
}

fn date(year: i32, month: u32, day: u32) -> Option<NaiveDate> {
    Some(NaiveDate::from_ymd(year, month, day))
}

fn oster_sonntag(year: i32) -> Option<NaiveDate> {
    let date = computus::gregorian(year).ok()?;
    NaiveDate::from_ymd_opt(date.year, date.month, date.day)
}

fn relative_to_easter_sunday(year: i32, days_offset: i64) -> Option<NaiveDate> {
    oster_sonntag(year).map(|date| date + Duration::days(days_offset))
}

fn bus_und_bettag(year: i32) -> Option<NaiveDate> {
    let reference_date = NaiveDate::from_ymd(year, 11, 23);
    let weekday_ordinal= i64::from(reference_date.weekday().num_days_from_monday());
    let duration_to_previous_wednesday = if weekday_ordinal < 3 {
        Duration::days(-(weekday_ordinal + 5))
    } else {
        Duration::days(2 - weekday_ordinal)
    };
    Some(reference_date + duration_to_previous_wednesday)
}

trait Holiday {
    fn to_date(&self, year: i32) -> Option<NaiveDate>;
}

impl Holiday for GermanHolidays {
    fn to_date(&self, year: i32) -> Option<NaiveDate> {
        match self {
            GermanHolidays::Neujahr => date(year, 1, 1),
            GermanHolidays::HeiligeDreiKoenige => date(year, 1, 6),
            GermanHolidays::Frauentag => date(year, 1, 8),
            GermanHolidays::Karfreitag => relative_to_easter_sunday(year, -2),
            GermanHolidays::Ostersonntag => oster_sonntag(year),
            GermanHolidays::Ostermontag => relative_to_easter_sunday(year, 1),
            GermanHolidays::ErsterMai => date(year, 5, 1),
            GermanHolidays::ChristiHimmelfahrt => relative_to_easter_sunday(year, 39),
            GermanHolidays::Pfingstsonntag => relative_to_easter_sunday(year, 49),
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
}

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

impl HolidayRegion<GermanHolidays> for Germany {
    fn holidays(&self) -> Vec<GermanHolidays> {
        BUNDESWEITE_FEIERTAGE
            .iter()
            .cloned()
            .chain(self.region_specific_holidays().iter().cloned())
            .collect()
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

#[cfg(test)]
mod tests {
    use crate::{GermanHolidays, GermanHolidays::*, Germany, Germany::*, HolidayRegion, ToHoliday, bus_und_bettag, date};
    use chrono::{Datelike, NaiveDate};

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
}
