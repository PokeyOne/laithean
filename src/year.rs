//! This module contains functionality relating to years.

#[cfg(test)]
mod tests;

use crate::month::Month;

/// Allow calculations of leap year.
pub trait Year {
    /// Determine if this year is a leap year or not.
    ///
    /// The calculation used is as follows. To be a leap year, the year must be
    /// a multiple of 4, but not 100, unless it is a multiple of 400.
    ///
    /// ## Verbose Psuedo Code
    /// ```text
    /// if mult of 4 {
    ///    if mult of 100 {
    ///        if mult of 400 {
    ///            leap year
    ///        } else {
    ///            not leap year
    ///        }
    ///    } else {
    ///        leap year
    ///    }
    /// } else {
    ///    not leap year
    /// }
    /// ```
    ///
    /// ## Truth Table
    ///
    /// Below is a logic truth table of when a year is a leap year. Note that
    /// some cases are missing because they are impossible, for example the year
    /// can not be a multiple of 100 or 400 if it is not a multiple of 4.
    /// | multiple of 4 | multiple of 100 | multiple of 400 | output |
    /// |---------------|-----------------|-----------------|--------|
    /// | 0 | 0 | 0 | 0 |
    /// | 1 | 0 | 0 | 1 |
    /// | 1 | 1 | 0 | 0 |
    /// | 1 | 1 | 1 | 1 |
    ///
    /// Using logic equations, and assigning the variables, a, b, and c to each
    /// of the input columns, we get the logic equation:
    /// ```text
    /// a * ((b * c) + (!b * !c)
    /// ```
    /// Or in programming syntax of logic `a && (b == c)`
    ///
    /// ## Concise Psuedo Code
    ///
    /// ```text
    /// if mult of 4 {
    ///    if (mult of 100) == (mult of 400) {
    ///        leap year
    ///    } else {
    ///        not leap year
    ///    }
    /// } else {
    ///    not leap year
    /// }
    /// ```
    fn is_leap_year(&self) -> bool;

    /// Return if the year is a multiple of the given value.
    fn is_mult(&self, value: Self) -> bool;

    /// The number of days in the year.
    ///
    /// This is not the same every year because of leap years.
    fn day_count(&self) -> u32;

    /// The index of the first day of the month in the year.
    ///
    /// This is not the same every year because of leap years.
    fn month_index(&self, month: Month) -> u32;
}

impl Year for u32 {
    fn is_leap_year(&self) -> bool {
        self.is_mult(4) && (
            self.is_mult(100) == self.is_mult(400)
        )
    }

    fn is_mult(&self, val: u32) -> bool {
        self % val == 0
    }

    fn day_count(&self) -> u32 {
        if self.is_leap_year() {
            366
        } else {
            365
        }
    }

    fn month_index(&self, month: Month) -> u32 {
        match month {
            Month::January => 0,
            Month::February => 31,
            Month::March => if self.is_leap_year() { 60 } else { 59 },
            Month::April => if self.is_leap_year() { 91 } else { 90 },
            Month::May => if self.is_leap_year() { 121 } else { 120 },
            Month::June => if self.is_leap_year() { 152 } else { 151 },
            Month::July => if self.is_leap_year() { 182 } else { 181 },
            Month::August => if self.is_leap_year() { 213 } else { 212 },
            Month::September => if self.is_leap_year() { 244 } else { 243 },
            Month::October => if self.is_leap_year() { 274 } else { 273 },
            Month::November => if self.is_leap_year() { 305 } else { 304 },
            Month::December => if self.is_leap_year() { 335 } else { 334 }
        }
    }
}
