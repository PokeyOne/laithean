use super::*;

#[test]
fn test_from_into_reversible() {
    let numbers: Vec<u8> = vec![0, 1, 2, 3, 4, 5, 6];

    // Convert to days
    let mut days: Vec<DayOfWeek> = vec![];
    for n in &numbers {
        days.push(DayOfWeek::from(*n));
    }

    // Convert back to numbers
    let mut result_numbers: Vec<u8> = vec![];
    for d in &days {
        result_numbers.push(u8::from(*d));
    }

    // Test
    assert_eq!(numbers, result_numbers);
}

#[test]
fn test_offset_zero() {
    assert_eq!(Monday, Monday.offset_by(0));
}

#[test]
fn test_offset_one_no_wrap() {
    assert_eq!(Tuesday, Monday.offset_by(1));
}

#[test]
fn test_offset_one_with_wrap() {
    assert_eq!(Monday, Sunday(false).offset_by(1));
}
