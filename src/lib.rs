use chrono::{Duration, NaiveDate};
use computus;

pub mod germany;

use germany::{GermanHoliday, GermanRegion};

fn date(year: i32, month: u32, day: u32) -> Option<NaiveDate> {
    Some(NaiveDate::from_ymd(year, month, day))
}

fn relative_to_easter_sunday(year: i32, days_offset: i64) -> Option<NaiveDate> {
    let easter_sunday = computus::gregorian(year).ok()?;
    let date = NaiveDate::from_ymd_opt(easter_sunday.year, easter_sunday.month, easter_sunday.day)?;
    Some(date + Duration::days(days_offset))
}

trait DateExt {
    fn is_holiday(&self, region: GermanRegion) -> bool;
    fn holiday(&self, region: GermanRegion) -> Option<GermanHoliday>;
}

impl DateExt for NaiveDate {
    fn is_holiday(&self, region: GermanRegion) -> bool {
        region.is_holiday(*self)
    }
    fn holiday(&self, region: GermanRegion) -> Option<GermanHoliday> {
        region.holiday_from_date(*self)
    }
}
