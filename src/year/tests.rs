use super::*;

#[test]
fn test_leap_year_not_mult_of_four() {
    assert!(!2001_u32.is_leap_year());
}

#[test]
fn test_leap_year_mult_of_four_not_hundred() {
    assert!(2004_u32.is_leap_year());
}

#[test]
fn test_leap_year_mult_of_four_hundred() {
    assert!(2000_u32.is_leap_year());
}

#[test]
fn test_leap_year_mult_of_four_and_hundred_not_four_hundred() {
    assert!(!2100_u32.is_leap_year());
}
