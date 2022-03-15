#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DayOfWeek {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday(bool)
}
use DayOfWeek::*;

impl DayOfWeek {
    pub fn scottish_gaelic(&self) -> &str {
        match self {
            Monday => "Diluain",
            Tuesday => "Dimàirt",
            Wednesday => "Diciadain",
            Thursday => "Diardaoin",
            Friday => "Dihaoine",
            Saturday => "Disathairne",
            Sunday(alt) => if *alt { "Là na Sàbaid" } else { "Didòmhnaich" }
        }
    }

    pub fn scottish_gaelic_abbreviation(&self) -> &str {
        match self {
            Monday => "DiL",
            Tuesday => "DiM",
            Wednesday => "DiC",
            Thursday => "Dia",
            Friday => "Dih",
            Saturday => "DiS",
            Sunday(_) => "DiD"
        }
    }
}

impl From<u8> for DayOfWeek {
    fn from(item: u8) -> DayOfWeek {
        let item = item % 7;

        match item {
            0 => Monday,
            1 => Tuesday,
            2 => Wednesday,
            3 => Thursday,
            4 => Friday,
            5 => Saturday,
            6 => Sunday(false),
            // Unreachable because mod(7) was used before the match
            _ => unreachable!()
        }
    }
}

impl From<DayOfWeek> for u8 {
    fn from(item: DayOfWeek) -> u8 {
        match item {
            Monday => 0,
            Tuesday => 1,
            Wednesday => 2,
            Thursday => 3,
            Friday => 4,
            Saturday => 5,
            Sunday(_) => 6,
        }
    }
}
