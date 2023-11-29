// src/lib.rs

use chrono::{NaiveDate, TimeZone, Utc};
use chrono_tz::Asia::Kathmandu;
mod date;
mod dump;
use date::convert_english_date_to_nepali;

/// Gets today's date in the Nepali calendar (Bikram Sambat).
///
/// # Examples
///
/// ```
/// use adtobs::get_todays_np_date;
///
/// let nepali_date = get_todays_np_date();
/// println!("Today's Nepali Date: {}", nepali_date);
/// ```
pub fn get_todays_np_date() -> String {
    let today = Utc::now().with_timezone(&Kathmandu);
    convert_english_date_to_nepali(&today)
}

/// Converts a Gregorian (English) date to a Nepali date (Bikram Sambat).
///
/// # Arguments
///
/// * `year` - The year in Gregorian calendar.
/// * `month` - The month in Gregorian calendar (1-12).
/// * `day` - The day in Gregorian calendar (1-31).
///
/// # Examples
///
/// ```
/// use adtobs::convert_ad_to_bs;
///
/// let nepali_date = convert_ad_to_bs(2023, 11, 29);
/// println!("Nepali Date: {}", nepali_date);
/// ```
pub fn convert_ad_to_bs(year: i32, month: u32, day: u32) -> String {
    let date = NaiveDate::from_ymd_opt(year, month, day)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();
    convert_english_date_to_nepali(&date)
}

/// Converts a UTC date string to a Nepali date (Bikram Sambat).
///
/// # Arguments
///
/// * `utc_string` - The date string in UTC format.
///
/// # Examples
///
/// ```
/// use adtobs::convert_utc_to_bs;
///
/// let utc_date = "2023-11-29T12:00:00Z";
/// let nepali_date = convert_utc_to_bs(utc_date);
/// println!("Nepali Date: {}", nepali_date);
/// ```
pub fn convert_utc_to_bs(utc_string: &str) -> String {
    let date = Utc::datetime_from_str(&Utc, utc_string, "%+")
        .unwrap()
        .with_timezone(&Kathmandu);
    convert_english_date_to_nepali(&date)
}
