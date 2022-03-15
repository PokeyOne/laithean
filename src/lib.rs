//! A Scottish Gaelic (Gàidhlig) date library.
//!
//! # Language Use
//! As this is a bi-lingual library, both languages are used in places on the
//! library. All variables, structures, fields are named in English, because
//! this library is meant to be as accessible to as many people as possible.
//! That being said, as much of the documentation as possible will be in both
//! languages.

pub mod month;

use month::Month;

pub struct Date {
    year: u64,
    month: Month,
    day: u8
}

impl Date {
    pub fn scottish_gaelic(&self) -> &str {
        todo!()
    }
}
