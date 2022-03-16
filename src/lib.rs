//! A Scottish Gaelic (Gàidhlig) date library.
//!
//! # Language Use
//! As this is a bi-lingual library, both languages are used in places on the
//! library. All variables, structures, fields are named in English, because
//! this library is meant to be as accessible to as many people as possible.
//! That being said, as much of the documentation as possible will be in both
//! languages.

#[cfg(test)]
mod tests;

pub mod year;
pub mod month;
pub mod days;

use month::Month;
use days::DayOfWeek;
use year::Year;

const REFERENCE_DAY_OF_WEEK: DayOfWeek = DayOfWeek::Tuesday;
const REFERENCE_DATE: Date = Date {
    year: 2022, month: Month::March, day_of_month_index: 14
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Date {
    year: u32,
    month: Month,
    day_of_month_index: u8
}

impl Date {
    pub fn new(year: u32, month: Month, day_of_month_index: u8) -> Date {
        assert!(day_of_month_index < month.day_count(year));

        Date { year, month, day_of_month_index }
    }

    pub fn today() -> Date {
        todo!()
    }

    pub fn scottish_gaelic(&self, _format: Option<&str>) -> String {
        // TODO: Accept a format string for customization

        format!(
            "{} {} {} {}",
            self.day_of_week().scottish_gaelic(),
            self.day_of_month_index + 1,
            self.month.scottish_gaelic(),
            self.year
        )
    }

    pub fn english(&self, _format: Option<&str>) -> String {
        format!(
            "{} {} {} {}",
            self.day_of_week().english(),
            self.day_of_month_index + 1,
            self.month.english(),
            self.year
        )
    }

    pub fn day_of_week(&self) -> DayOfWeek {
        let offset: i64 = REFERENCE_DATE.difference(self);
        REFERENCE_DAY_OF_WEEK.offset_by(offset)
    }

    /// The number of days from self until other. Negative if other is before
    /// self.
    pub fn difference(&self, other: &Date) -> i64 {
        // Make sure other is after self.
        if self.after(other) {
            return -1 * other.difference(self);
        }

        // Sum all the years in between.
        let mut days: u32 = 0;
        for y in (self.year)..(other.year) {
            days += y.day_count();
        }
        // And the actual days within the year.
        days += other.day_index_in_year();
        days -= self.day_index_in_year();

        return days as i64;
    }

    pub fn after(&self, other: &Date) -> bool {
        if self.year > other.year {
            true
        } else if self.year < other.year {
            false
        } else {
            // Same years

            if self.month.after(&other.month) {
                true
            } else if self.month.before(&other.month) {
                false
            } else {
                // Same months

                self.day_of_month_index > other.day_of_month_index
            }
        }
    }

    /// The index of the day in the year.
    ///
    /// # Examples
    /// ```
    /// use laithean::{Date, month::Month};
    ///
    /// let d = Date::new(2022, Month::January, 0);
    /// assert_eq!(d.day_index_in_year(), 0);
    ///
    /// let d = Date::new(2022, Month::February, 0);
    /// assert_eq!(d.day_index_in_year(), 31);
    /// ```
    pub fn day_index_in_year(&self) -> u32 {
        self.year.month_index(self.month) + (self.day_of_month_index as u32)
    }
}

impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.scottish_gaelic(None))?;
        Ok(())
    }
}
