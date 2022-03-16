use crate::year::Year;

pub const MONTHS: [Month; 12] = [
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
];

#[derive(Debug, Clone, Copy, PartialEq)]
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

    pub fn day_count(&self, year: u32) -> u8 {
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

    pub fn after(&self, other: &Month) -> bool {
        (*self as u8) > (*other as u8)
    }

    pub fn before(&self, other: &Month) -> bool {
        other.after(self)
    }
}

impl From<Month> for u8 {
    fn from(item: Month) -> u8 {
        match item {
            January => 0,
            February => 1,
            March => 2,
            April => 3,
            May => 4,
            June => 5,
            July => 6,
            August => 7,
            September => 8,
            October => 9,
            November => 10,
            December => 11
        }
    }
}

impl From<u8> for Month {
    fn from(item: u8) -> Month {
        match item {
            0 => January,
            1 => February,
            2 => March,
            3 => April,
            4 => May,
            5 => June,
            6 => July,
            7 => August,
            8 => September,
            9 => October,
            10 => November,
            11 => December,
            _ => Month::from(item % 12)
        }
    }
}
