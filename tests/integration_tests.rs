// tests/integration_tests.rs

use adtobs::{convert_ad_to_bs, convert_utc_to_bs, get_todays_np_date};

#[test]
fn test_get_todays_np_date() {
    // Test get_todays_np_date function
    let today = get_todays_np_date();
    assert!(!today.is_empty()); // Assuming a non-empty string is returned
}

#[test]
fn test_convert_ad_to_bs() {
    // Test convert_ad_to_bs function
    let converted_date = convert_ad_to_bs(2023, 11, 29);
    assert_eq!(converted_date, "2080 Mangsir 13, Wednesday");
}

#[test]
fn test_convert_utc_to_bs() {
    // Test convert_utc_to_bs function
    let utc_date = "2023-11-29T12:00:00Z";
    let converted_date = convert_utc_to_bs(utc_date);
    assert_eq!(converted_date, "2080 Mangsir 13, Wednesday");
}
