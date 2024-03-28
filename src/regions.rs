use chrono::{Datelike, NaiveDate};

/// Represents all regions and their public holidays within Germany.
///
/// Holidays guaranteed to take place on sundays, e.g. easter sunday, are excluded by default.
/// However, holidays with a fixed date can still fall on a sunday.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GermanRegion {
    BadenWuerttemberg,
    /// * The Augsburger Friedensfest only applies to Augsburg.
    ///   It is excluded by default, but can be calculated via `GermanHoliday::AugsburgerFriedensfest`.
    /// * Mariä Himmelfahrt only applies to communities with a catholic majority.
    ///   Since this is the case in the majority of communities, it is included by default.
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
    /// Fronleichnam applies only to a minority of communities and has been excluded by default.
    /// It can be manually calculated via `GermanHoliday::Fronleichnam`.
    Sachsen,
    SachsenAnhalt,
    SchleswigHolstein,
    /// Fronleichnam applies only to a minority of communities and has been excluded by default.
    /// It can be manually calculated via `GermanHoliday::Fronleichnam`.
    Thueringen,
}

use crate::holidays::GermanHoliday;
use crate::holidays::GermanHoliday::*;
use crate::regions::GermanRegion::*;

impl GermanRegion {
    /// Returns all public holidays in the given year.
    /// Holidays guaranteed to take place on sundays, e.g. easter sunday, are excluded by default.
    ///
    /// For years before 1995 this list will be empty.
    pub fn holidays_in_year(&self, year: i32) -> Vec<GermanHoliday> {
        if year < 1995 {
            return Vec::new();
        }
        let mut holidays = Vec::new();
        holidays.extend_from_slice(BUNDESWEITE_FEIERTAGE);
        holidays.extend_from_slice(self.region_specific_holidays(year));
        if year == 2017 && !holidays.contains(&Reformationstag) {
            // BW: https://www.landesrecht-bw.de/perma?d=jlr-FeiertGBWV1P1a
            // BY: https://www.bayern.landtag.de/www/ElanTextAblage_WP17/Drucksachen/Folgedrucksachen/0000007000/0000007463.pdf
            // BE: https://gesetze.berlin.de/bsbe/document/aiz-jlr-FeiertGBErahmen%4020151025/part/x
            // HE: https://www.rv.hessenrecht.hessen.de/bshe/document/jlr-RefT2017VHErahmen/part/X
            // NW: https://www.landtag.nrw.de/portal/WWW/dokumentenarchiv/Dokument?Id=XMMGVB1528%7C496%7C496
            // RP: https://web.archive.org/web/20160305005630/https://www.rlp.de/fr/aktuelles/einzelansicht/news/detail/News/zusaetzlicher-feiertag-2017/
            // SL: https://web.archive.org/web/20160306062414/http://sl.juris.de/cgi-bin/landesrecht.py?d=http%3A%2F%2Fsl.juris.de%2Fsl%2Fgesamt%2FRefT2017V_SL.htm
            holidays.push(Reformationstag);
        }
        holidays
    }

    fn region_specific_holidays(&self, year: i32) -> &'static [GermanHoliday] {
        match self {
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
            Bremen => {
                if year >= 2017 {
                    &[Reformationstag]
                } else {
                    &[]
                }
            }
            Hamburg => {
                if year >= 2017 {
                    &[Reformationstag]
                } else {
                    &[]
                }
            }
            Hessen => &[Fronleichnam],
            MechlenburgVorpommern => {
                if year >= 2023 {
                    &[Frauentag, Reformationstag]
                } else {
                    &[Reformationstag]
                }
            }
            Niedersachsen => {
                if year >= 2017 {
                    &[Reformationstag]
                } else {
                    &[]
                }
            }
            NordrheinWestfalen => &[Fronleichnam, Allerheiligen],
            RheinlandPfalz => &[Fronleichnam, Allerheiligen],
            Saarland => &[Fronleichnam, MariaeHimmelfahrt, Allerheiligen],
            Sachsen => &[Reformationstag, BussUndBettag],
            SachsenAnhalt => &[HeiligeDreiKoenige, Reformationstag],
            SchleswigHolstein => {
                if year >= 2017 {
                    &[Reformationstag]
                } else {
                    &[]
                }
            }
            Thueringen => {
                if year >= 2019 {
                    &[Weltkindertag, Reformationstag]
                } else {
                    &[Reformationstag]
                }
            }
        }
    }

    /// Returns all holidays and their dates in the given year.
    /// Holidays guaranteed to take place on sundays, e.g. easter sunday, are excluded by default.
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

    /// Checks if a given date is a public holiday in the specific region.
    ///
    /// Always `false` for dates before 1995.
    pub fn is_holiday(&self, date: NaiveDate) -> bool {
        self.holiday_from_date(date).is_some()
    }

    /// Returns the holiday for a specific date if the date is a holiday in the specific region.
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
    use proptest::prelude::*;

    #[test]
    fn singular_example_holiday() {
        let date = NaiveDate::from_ymd(2018, 1, 1);
        assert!(date.is_public_holiday_in(Bayern));
        assert_eq!(Some(Neujahr), date.public_holiday_in(Bayern));
    }

    proptest! {
    #[test]
    fn total_number_holidays(year in 2023i32..) {
        let number_holidays = |region: GermanRegion| region.holidays_in_year(year).len();
        assert_eq!(12, number_holidays(BadenWuerttemberg));
        assert_eq!(13, number_holidays(Bayern));
        assert_eq!(10, number_holidays(Berlin));
        assert_eq!(10, number_holidays(Brandenburg));
        assert_eq!(10, number_holidays(Bremen));
        assert_eq!(10, number_holidays(Hamburg));
        assert_eq!(10, number_holidays(Hessen));
        assert_eq!(11, number_holidays(MechlenburgVorpommern));
        assert_eq!(10, number_holidays(Niedersachsen));
        assert_eq!(11, number_holidays(NordrheinWestfalen));
        assert_eq!(11, number_holidays(RheinlandPfalz));
        assert_eq!(12, number_holidays(Saarland));
        assert_eq!(11, number_holidays(Sachsen));
        assert_eq!(11, number_holidays(SachsenAnhalt));
        assert_eq!(10, number_holidays(SchleswigHolstein));
        assert_eq!(11, number_holidays(Thueringen));
    }
    }

    #[test]
    fn frauentag_in_berlin_since_2019() {
        assert!(!Berlin.holidays_in_year(2018).contains(&Frauentag));
        assert_eq!(
            None,
            NaiveDate::from_ymd(2018, 3, 8).public_holiday_in(Berlin)
        );
        assert!(Berlin.holidays_in_year(2019).contains(&Frauentag));
        assert_eq!(
            Some(Frauentag),
            NaiveDate::from_ymd(2019, 3, 8).public_holiday_in(Berlin)
        );
    }

    proptest! {
    #[test]
    fn only_provide_holidays_after_1995(year in -2999i32..1995) {
        assert!(BadenWuerttemberg.holidays_in_year(year).is_empty());
    }
    }

}
