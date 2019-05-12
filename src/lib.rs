//! Small crate to calculate public holidays for each German federal state.
//!
//! This crate can calculate all *reoccurring* German holidays, which exist since 1995.
//!
//! There are some weird edge cases depending on the respective region,
//! see `GermanRegion` for more details.
//! A comprehensive overview can be found within the German Wikipedia
//! [Gesetzliche Feiertage in Deutschland](https://de.wikipedia.org/wiki/Gesetzliche_Feiertage_in_Deutschland).
use chrono::{Datelike, NaiveDate};

mod holidays;
mod regions;

pub use holidays::GermanHoliday;
pub use regions::GermanRegion;

/// Provides convenience methods for datelike data structures like `NaiveDate`.
pub trait DateExt {
    /// True if date is a holiday within the specified region.
    ///
    /// Always `false` for dates before 1995.
    fn is_public_holiday_in(&self, region: GermanRegion) -> bool;

    /// Returns the holiday if given date is a public holiday.
    ///
    /// Always `None` for dates before 1995.
    fn public_holiday_in(&self, region: GermanRegion) -> Option<GermanHoliday>;

    /// True if date falls on the date of the given holiday.
    fn is_holiday(&self, holiday: GermanHoliday) -> bool;
}

impl DateExt for NaiveDate {
    fn is_public_holiday_in(&self, region: GermanRegion) -> bool {
        region.is_holiday(*self)
    }
    fn public_holiday_in(&self, region: GermanRegion) -> Option<GermanHoliday> {
        region.holiday_from_date(*self)
    }
    fn is_holiday(&self, holiday: GermanHoliday) -> bool {
        let holiday_date = holiday.date(self.year());
        Some(*self) == holiday_date
    }
}
