//! Small crate to calculate public holidays for each German federal state.
//!
//! This crate can calculate all German public holidays, which exist since 1995.
//! Holidays guaranteed to take place on sundays, e.g. easter sunday, are excluded by default.
//! However, some holidays occuring on a fixed date can still fall on a sunday.
//!
//! There are some weird edges depending on the respective region,
//! see `GermanRegion` for more details.
//!
//! A comprehensive overview can be found within the German Wikipedia
//! [Gesetzliche Feiertage in Deutschland](https://de.wikipedia.org/wiki/Gesetzliche_Feiertage_in_Deutschland).
use chrono::NaiveDate;

mod holidays;
mod regions;

pub use holidays::GermanHoliday;
pub use regions::GermanRegion;

/// Provides convenience methods for datelike data structures like `NaiveDate`.
pub trait DateExt {
    /// True if date is a holiday within the German region.
    ///
    /// Always `false` for dates before 1995.
    fn is_holiday(&self, region: GermanRegion) -> bool;

    /// Returns the holiday if given date is a public holiday.
    ///
    /// Always `None` for dates before 1995.
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
