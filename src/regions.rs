use chrono::{Datelike, NaiveDate};

/// Represents all regions and there holidays within Germany.
pub enum GermanRegion {
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

use crate::holidays::GermanHoliday;
use crate::holidays::GermanHoliday::*;
use crate::regions::GermanRegion::*;

impl GermanRegion {
    /// Returns all holidays in the given year.
    ///
    /// For years before 1995 this list will be empty.
    pub fn holidays_in_year(&self, year: i32) -> Vec<GermanHoliday> {
        if year < 1995 {
            return Vec::new();
        }
        let mut holidays = Vec::new();
        holidays.extend_from_slice(BUNDESWEITE_FEIERTAGE);
        let region_specific_holidays: &'static [GermanHoliday] = match self {
            BadenWuerttemberg => &[HeiligeDreiKoenige, Fronleichnam, Allerheiligen],
            Bayern => &[
                HeiligeDreiKoenige,
                Fronleichnam,
                MariaeHimmelfahrt,
                Allerheiligen,
            ],
            Berlin => {
                if year >= 2019 {
                    &[Frauentag]
                } else {
                    &[]
                }
            }
            Brandenburg => &[Reformationstag],
            Bremen => &[Reformationstag],
            Hamburg => &[Reformationstag],
            Hessen => &[Fronleichnam],
            MechlenburgVorpommern => &[Reformationstag],
            Niedersachsen => &[Reformationstag],
            NordrheinWestfalen => &[Fronleichnam, Allerheiligen],
            RheinlandPfalz => &[Fronleichnam, Allerheiligen],
            Saarland => &[Fronleichnam, MariaeHimmelfahrt, Allerheiligen],
            Sachsen => &[Reformationstag, BussUndBettag],
            SachsenAnhalt => &[HeiligeDreiKoenige, Reformationstag],
            SchleswigHolstein => &[Reformationstag],
            Thueringen => &[Weltkindertag, Reformationstag],
        };
        holidays.extend_from_slice(region_specific_holidays);
        holidays
    }

    /// Returns all holidays and their dates in the given year.
    ///
    /// For years before 1995 this list will be empty.
    pub fn holiday_dates_in_year(&self, year: i32) -> Vec<(NaiveDate, GermanHoliday)> {
        let mut holiday_dates: Vec<(NaiveDate, GermanHoliday)> = self
            .holidays_in_year(year)
            .into_iter()
            .flat_map(|holiday| holiday.date(year).map(|date| (date, holiday)))
            .collect();
        holiday_dates.sort_unstable_by_key(|(date, _)| *date);
        holiday_dates
    }

    /// Checks if a given date is a holiday in the specific region.
    ///
    /// Always `false` for dates before 1995.
    pub fn is_holiday(&self, date: NaiveDate) -> bool {
        self.holiday_from_date(date).is_some()
    }

    /// Returns the holiday for a specific date if the date is a holiday.
    ///
    /// Always `None` for dates before 1995.
    pub fn holiday_from_date(&self, date: NaiveDate) -> Option<GermanHoliday> {
        self.holidays_in_year(date.year())
            .into_iter()
            .find(|holiday| holiday.date(date.year()) == Some(date))
    }
}

const BUNDESWEITE_FEIERTAGE: &'static [GermanHoliday] = &[
    Neujahr,
    Karfreitag,
    Ostermontag,
    ErsterMai,
    ChristiHimmelfahrt,
    Pfingstmontag,
    TagDerDeutschenEinheit,
    ErsterWeihnachtsfeiertag,
    ZweiterWeihnachtsfeiertag,
];

#[cfg(test)]
mod tests {
    use crate::regions::GermanHoliday::*;
    use crate::regions::GermanRegion;
    use crate::regions::GermanRegion::*;
    use crate::DateExt;
    use chrono::NaiveDate;

    #[test]
    fn neujahr_feiertag_in_bayern() {
        let date = NaiveDate::from_ymd(2018, 01, 01);
        assert!(date.is_holiday(Bayern));
        assert_eq!(Some(Neujahr), date.holiday(Bayern));
    }

    #[test]
    fn total_number_holidays() {
        let number_holidays = |region: GermanRegion| region.holidays_in_year(2019).len();
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
    fn frauntag_in_berlin_since_2019() {
        assert!(!Berlin.holidays_in_year(2018).contains(&Frauentag));
        assert_eq!(None, NaiveDate::from_ymd(2018, 3, 8).holiday(Berlin));
        assert!(Berlin.holidays_in_year(2019).contains(&Frauentag));
        assert_eq!(
            Some(Frauentag),
            NaiveDate::from_ymd(2019, 3, 8).holiday(Berlin)
        );
    }

    #[test]
    fn only_provide_holidays_after_1995() {
        assert!(BadenWuerttemberg.holidays_in_year(1994).is_empty());
    }

}
