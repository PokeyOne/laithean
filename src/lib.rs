//! A Scottish Gaelic (Gàidhlig) date library.
//!
//! # Language Use
//! As this is a bi-lingual library, both languages are used in places on the
//! library. All variables, structures, fields are named in English, because
//! this library is meant to be as accessible to as many people as possible.
//! That being said, as much of the documentation as possible will be in both
//! languages.

pub mod month;
pub mod days;

use month::Month;
use days::DayOfWeek;

const DAY_OF_WEEK_MAR_15_2022: DayOfWeek = DayOfWeek::Tuesday;

pub struct Date {
    year: u64,
    month: Month,
    day_of_month: u8
}

impl Date {
    pub fn scottish_gaelic(&self, _format: &str) -> String {
        // TODO: Accept a format string for customization

        format!(
            "{} {} {} {}",
            self.day_of_week().scottish_gaelic(),
            self.day_of_month,
            self.month.scottish_gaelic(),
            self.year
        )
    }

    pub fn day_of_week(&self) -> DayOfWeek {
        todo!()
    }

    /// A unique integer representation of the date relative to March 15, 2022.
    ///
    /// This date was chosen because the function was written on this date, and
    /// the day of the week was known.
    fn day_of_ref(&self) -> i64 {
        todo!()
    }
}
