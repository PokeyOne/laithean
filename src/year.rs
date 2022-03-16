//! This module contains functionality relating to years.

#[cfg(test)]
mod tests;

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
}
