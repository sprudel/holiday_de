use chrono::Duration;
use chrono::{Datelike, NaiveDate};
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
    panic!("unimplemented")
}

fn relative_to_easter_sunday(year: i32, days_offset: i64) -> Option<NaiveDate> {
    oster_sonntag(year).map(|date| date + Duration::days(days_offset))
}

fn bus_und_bettag(year: i32) -> Option<NaiveDate> {
    panic!("unimplemented")
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
    GermanHolidays::Ostersonntag,
    GermanHolidays::Ostermontag,
    GermanHolidays::ErsterMai,
    GermanHolidays::ChristiHimmelfahrt,
    GermanHolidays::Pfingstsonntag,
    GermanHolidays::Pfingstmontag,
    GermanHolidays::TagDerDeutschenEinheit,
    GermanHolidays::ErsterWeihnachtsfeiertag,
    GermanHolidays::ZweiterWeihnachtsfeiertag,
];

impl Germany {
    fn holidays(&self) -> impl Iterator<Item = GermanHolidays> {
        BUNDESWEITE_FEIERTAGE
            .iter()
            .cloned()
            .chain(self.region_specific_holidays().iter().cloned())
    }

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

trait Region<H>
where
    H: Holiday,
{
    fn is_holiday(&self, date: NaiveDate) -> bool;
    fn holiday_from_date(&self, date: NaiveDate) -> Option<H>;
    fn holidays_in_year(&self, year: i32) -> BTreeMap<NaiveDate, H>;
}

impl Region<GermanHolidays> for Germany {
    fn is_holiday(&self, date: NaiveDate) -> bool {
        self.holidays()
            .any(|holiday| holiday.to_date(date.year()) == Some(date))
    }
    fn holiday_from_date(&self, date: NaiveDate) -> Option<GermanHolidays> {
        self.holidays()
            .find(|holiday| holiday.to_date(date.year()) == Some(date))
    }
    fn holidays_in_year(&self, year: i32) -> BTreeMap<NaiveDate, GermanHolidays> {
        self.holidays()
            .map(|holiday| (holiday.to_date(year), holiday))
            .filter(|(date, _)| date.is_some())
            .map(|(date, holiday)| (date.unwrap(), holiday))
            .collect()
    }
}

trait TryHoliday<R, H>
where
    H: Holiday,
    R: Region<H>,
{
    fn is_holiday(&self, region: R) -> bool;
    fn holiday(&self, region: R) -> Option<H>;
}

impl<R, H> TryHoliday<R, H> for NaiveDate
where
    H: Holiday,
    R: Region<H>,
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
    use crate::{GermanHolidays::*, Germany::*, TryHoliday};
    use chrono::{Datelike, NaiveDate};

    #[test]
    fn neujahr_feiertag_in_bayern() {
        let date = NaiveDate::from_ymd(2018, 01, 01);
        assert!(date.is_holiday(Bayern));
        assert_eq!(date.holiday(Bayern), Some(Neujahr))
    }
}
