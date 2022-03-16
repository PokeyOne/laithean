use crate::year::Year;

pub enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December
}
use Month::*;

impl Month {
    /// Get the name of the month.
    ///
    /// This does not include any definite article ('the' in English), therefore
    /// this should be used inside of a date for example. If the definite
    /// article is needed, then you may call the
    /// [`Month::scottish_gaelic_with_def_art()`] method.
    ///
    /// # Examples
    /// ```
    /// # use laithean::month::Month;
    /// assert_eq!("Faoilleach", Month::January.scottish_gaelic());
    /// ```
    pub fn scottish_gaelic(&self) -> &str {
        match self {
            January => "Faoilleach",
            February => "Gearran",
            March => "Màrt",
            April => "Giblean",
            May => "Cèitean",
            June => "Ògmhios",
            July => "Iuchar",
            August => "Lùnastal",
            September => "Sultain",
            October => "Dàmhair",
            November => "Samhain",
            December => "Dùbhlachd"
        }
    }

    /// Get the name of the month with the definite article.
    ///
    /// The definite article is the "the" in English.
    ///
    /// # Parameters
    /// - `cap` [bool] - Whether or not to capitalize the definite article.
    ///
    /// # Examples
    /// ```
    /// # use laithean::month::Month;
    /// let uncapitalized = Month::January.scottish_gaelic_with_def_art(false);
    /// assert_eq!("am Faoilleach", uncapitalized);
    /// let capitalized = Month::January.scottish_gaelic_with_def_art(true);
    /// assert_eq!("Am Faoilleach", capitalized);
    /// ```
    pub fn scottish_gaelic_with_def_art(&self, cap: bool) -> &str {
        match self {
            January => if cap { "Am Faoilleach" } else { "am Faoilleach" },
            February => if cap { "An Gearran" } else { "an Gearran" },
            March => if cap { "Am Màrt" } else { "am Màrt" },
            April => if cap { "An Giblean" } else { "an Giblean" },
            May => if cap { "An Cèitean" } else { "an Cèitean" },
            June => if cap { "An t-Ògmhios" } else { "an t-Ògmhios" },
            July => if cap { "An t-Iuchar" } else { "an t-Iuchar" },
            August => if cap { "An Lùnastal" } else { "an Lùnastal" },
            September => if cap { "An t-Sultain" } else { "an Sultain" },
            October => if cap { "An Dàmhair" } else { "an Dàmhair" },
            November => if cap { "An t-Samhain" } else { "an t-Samhain" },
            December => if cap { "An Dùbhlachd" } else { "an Dùbhlachd" },
        }
    }

    pub fn day_count(&self, year: u32) -> u64 {
        match self {
            January => 31,
            February => if year.is_leap_year() { 29 } else { 28 },
            March => 31,
            April => 30,
            May => 31,
            June => 30,
            July => 31,
            August => 31,
            September => 30,
            October => 31,
            November => 30,
            December => 31
        }
    }
}
