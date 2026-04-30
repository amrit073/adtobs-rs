// tests/integration_tests.rs

use adtobs::{Month, NepaliDate, NepaliDateError};
use chrono::{NaiveDate, Weekday};

#[test]
fn from_gregorian_ymd_matches_known_conversion() {
    let bs = NepaliDate::from_gregorian_ymd(2023, 11, 29).unwrap();
    assert_eq!(bs.year(), 2080);
    assert_eq!(bs.month(), Month::Mangsir);
    assert_eq!(bs.day(), 13);
    assert_eq!(bs.weekday(), Weekday::Wed);
    assert_eq!(bs.to_string(), "2080 Mangsir 13, Wednesday");
}

#[test]
fn try_from_naive_date_works() {
    let nd = NaiveDate::from_ymd_opt(2023, 11, 29).unwrap();
    let bs = NepaliDate::try_from(nd).unwrap();
    assert_eq!(bs.to_string(), "2080 Mangsir 13, Wednesday");
}

#[test]
fn from_rfc3339_uses_nepal_time() {
    let bs: NepaliDate = "2023-11-29T12:00:00Z".parse().unwrap();
    assert_eq!(bs.to_string(), "2080 Mangsir 13, Wednesday");
}

#[test]
fn out_of_range_year_returns_error() {
    let err = NepaliDate::from_gregorian_ymd(1800, 1, 1).unwrap_err();
    assert!(matches!(err, NepaliDateError::OutOfRange { .. }));
}

#[test]
fn invalid_calendar_date_returns_error() {
    let err = NepaliDate::from_gregorian_ymd(2023, 2, 30).unwrap_err();
    assert!(matches!(err, NepaliDateError::InvalidGregorian { .. }));
}

#[test]
fn unparseable_rfc3339_returns_error() {
    let err = NepaliDate::from_rfc3339("not a date").unwrap_err();
    assert!(matches!(err, NepaliDateError::Parse(_)));
}

#[test]
fn today_does_not_panic() {
    // Just make sure today() succeeds and produces a non-empty rendering.
    let today = NepaliDate::today();
    assert!(!today.to_string().is_empty());
}

#[test]
fn from_bs_ymd_round_trips_with_ad() {
    let bs = NepaliDate::from_bs_ymd(2080, 8, 13).unwrap();
    assert_eq!(bs.year(), 2080);
    assert_eq!(bs.month(), Month::Mangsir);
    assert_eq!(bs.day(), 13);
    assert_eq!(bs.weekday(), Weekday::Wed);
    assert_eq!(bs.to_string(), "2080 Mangsir 13, Wednesday");

    let ad = bs.to_gregorian();
    assert_eq!(ad, NaiveDate::from_ymd_opt(2023, 11, 29).unwrap());

    // Round-trip back to the same NepaliDate.
    assert_eq!(NepaliDate::try_from(ad).unwrap(), bs);
}

#[test]
fn bs_baseline_maps_to_1944_01_01() {
    let bs = NepaliDate::from_bs_ymd(2000, 9, 17).unwrap();
    assert_eq!(bs.weekday(), Weekday::Sat);
    assert_eq!(
        bs.to_gregorian(),
        NaiveDate::from_ymd_opt(1944, 1, 1).unwrap()
    );
}

#[test]
fn naive_date_from_nepali_date_works() {
    let bs = NepaliDate::from_bs_ymd(2080, 8, 13).unwrap();
    let ad: NaiveDate = bs.into();
    assert_eq!(ad, NaiveDate::from_ymd_opt(2023, 11, 29).unwrap());
}

#[test]
fn invalid_bs_dates_return_errors() {
    // Out of range year (table covers 2000..=2090).
    assert!(matches!(
        NepaliDate::from_bs_ymd(1999, 1, 1),
        Err(NepaliDateError::OutOfRange { .. })
    ));
    assert!(matches!(
        NepaliDate::from_bs_ymd(2091, 1, 1),
        Err(NepaliDateError::OutOfRange { .. })
    ));
    // Month out of range.
    assert!(matches!(
        NepaliDate::from_bs_ymd(2080, 13, 1),
        Err(NepaliDateError::InvalidBs { .. })
    ));
    // Day exceeds that month's length (Mangsir 2080 has 30 days, not 31).
    assert!(matches!(
        NepaliDate::from_bs_ymd(2080, 8, 31),
        Err(NepaliDateError::InvalidBs { .. })
    ));
    // Day zero.
    assert!(matches!(
        NepaliDate::from_bs_ymd(2080, 8, 0),
        Err(NepaliDateError::InvalidBs { .. })
    ));
}

#[test]
fn boundary_years_are_supported() {
    // First and last BS years in the table should both be constructible.
    let first = NepaliDate::from_bs_ymd(2000, 1, 1).unwrap();
    let last = NepaliDate::from_bs_ymd(2090, 12, 30).unwrap();
    // And the corresponding Gregorian dates should be sensible.
    assert!(first.to_gregorian() < last.to_gregorian());
}

#[test]
fn round_trip_across_a_range_of_dates() {
    // Every 17th of every month from BS 2070 to 2085 — covers leap and
    // non-leap Gregorian years and many month boundaries.
    for year in 2070..=2085 {
        for month in 1u32..=12 {
            let bs = NepaliDate::from_bs_ymd(year, month, 17).unwrap();
            let ad = bs.to_gregorian();
            let back = NepaliDate::try_from(ad).unwrap();
            assert_eq!(back, bs, "round-trip failed for BS {year}-{month}-17");
        }
    }
}

#[test]
fn month_helpers_round_trip() {
    for n in 1u8..=12 {
        let m = Month::from_number(n).unwrap();
        assert_eq!(m.number(), n);
    }
    assert_eq!(Month::from_number(0), None);
    assert_eq!(Month::from_number(13), None);
    assert_eq!(Month::Mangsir.name(), "Mangsir");
}

// Backwards-compatibility tests for the deprecated free functions.
#[test]
#[allow(deprecated)]
fn legacy_get_todays_np_date_still_works() {
    let today = adtobs::get_todays_np_date();
    assert!(!today.is_empty());
}

#[test]
#[allow(deprecated)]
fn legacy_convert_ad_to_bs_still_works() {
    assert_eq!(
        adtobs::convert_ad_to_bs(2023, 11, 29),
        "2080 Mangsir 13, Wednesday"
    );
}

#[test]
#[allow(deprecated)]
fn legacy_convert_utc_to_bs_still_works() {
    assert_eq!(
        adtobs::convert_utc_to_bs("2023-11-29T12:00:00Z"),
        "2080 Mangsir 13, Wednesday"
    );
}

#[test]
#[allow(deprecated)]
fn legacy_invalid_input_returns_error_string() {
    // Old behaviour preserved: bad inputs come back as the literal "Invalid date !".
    assert_eq!(adtobs::convert_ad_to_bs(1800, 1, 1), "Invalid date !");
    assert_eq!(adtobs::convert_utc_to_bs("not a date"), "Invalid date !");
}
