use super::*;

#[test]
fn test_day_of_week_at_ref() {
    let date = REFERENCE_DATE.clone();

    assert_eq!(REFERENCE_DAY_OF_WEEK, date.day_of_week());
}

#[test]
fn test_date_diff_with_ref() {
    assert_eq!(0, REFERENCE_DATE.difference(&REFERENCE_DATE));
}
