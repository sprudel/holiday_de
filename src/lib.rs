use chrono::{Datelike, Duration, NaiveDate, Weekday};
use computus;
use std::collections::BTreeMap;
use std::fmt::Display;

mod germany;

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
