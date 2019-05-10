use chrono::NaiveDate;

mod holidays;
mod regions;

pub use holidays::GermanHoliday;
pub use regions::GermanRegion;

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
