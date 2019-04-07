use chrono::{Datelike, Duration, NaiveDate};
use computus;

pub mod germany;

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
    fn holidays_in_year(&self, year: i32) -> Vec<H>;
    fn holiday_dates_in_year(&self, year: i32) -> Vec<(NaiveDate, H)> {
        let mut holiday_dates: Vec<(NaiveDate, H)> = self
            .holidays_in_year(year)
            .into_iter()
            .flat_map(|holiday| holiday.to_date(year).map(|date| (date, holiday)))
            .collect();
        holiday_dates.sort_unstable_by_key(|(date, _)| *date);
        holiday_dates
    }
    fn is_holiday(&self, date: NaiveDate) -> bool {
        self.holiday_from_date(date).is_some()
    }
    fn holiday_from_date(&self, date: NaiveDate) -> Option<H> {
        self.holidays_in_year(date.year())
            .into_iter()
            .find(|holiday| holiday.to_date(date.year()) == Some(date))
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
