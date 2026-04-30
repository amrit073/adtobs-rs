//! Convert dates between the Gregorian (English) calendar and the Bikram
//! Sambat (Nepali) calendar, in both directions.
//!
//! The recommended entry point is the [`NepaliDate`] type:
//!
//! ```
//! use adtobs::NepaliDate;
//! use chrono::NaiveDate;
//!
//! // AD → BS
//! let bs = NepaliDate::from_gregorian_ymd(2023, 11, 29).unwrap();
//! assert_eq!(bs.to_string(), "2080 Mangsir 13, Wednesday");
//!
//! // BS → AD
//! let bs = NepaliDate::from_bs_ymd(2080, 8, 13).unwrap();
//! assert_eq!(bs.to_gregorian(), NaiveDate::from_ymd_opt(2023, 11, 29).unwrap());
//! ```
//!
//! The free functions [`get_todays_np_date`], [`convert_ad_to_bs`] and
//! [`convert_utc_to_bs`] are kept for backwards compatibility but are
//! deprecated; prefer the [`NepaliDate`] API.

mod dump;
mod nepali_date;

pub use nepali_date::{Month, NepaliDate, NepaliDateError};

/// Returns today's Nepali (Bikram Sambat) date as a formatted string.
///
/// # Examples
///
/// ```
/// # #[allow(deprecated)]
/// # {
/// use adtobs::get_todays_np_date;
/// let nepali_date = get_todays_np_date();
/// println!("Today's Nepali Date: {}", nepali_date);
/// # }
/// ```
#[deprecated(since = "0.2.0", note = "use `NepaliDate::today().to_string()` instead")]
pub fn get_todays_np_date() -> String {
    NepaliDate::today().to_string()
}

/// Converts a Gregorian (English) date to a Nepali (Bikram Sambat) date string.
///
/// Returns the literal `"Invalid date !"` for out-of-range or non-existent
/// inputs, matching the original 0.1.x behaviour.
///
/// # Examples
///
/// ```
/// # #[allow(deprecated)]
/// # {
/// use adtobs::convert_ad_to_bs;
/// let nepali_date = convert_ad_to_bs(2023, 11, 29);
/// assert_eq!(nepali_date, "2080 Mangsir 13, Wednesday");
/// # }
/// ```
#[deprecated(
    since = "0.2.0",
    note = "use `NepaliDate::from_gregorian_ymd(year, month, day)` instead"
)]
pub fn convert_ad_to_bs(year: i32, month: u32, day: u32) -> String {
    match NepaliDate::from_gregorian_ymd(year, month, day) {
        Ok(d) => d.to_string(),
        Err(_) => String::from("Invalid date !"),
    }
}

/// Converts an RFC 3339 / ISO 8601 timestamp string to a Nepali date string.
///
/// Returns the literal `"Invalid date !"` if the string is not a parseable
/// timestamp or the resulting date is out of range, matching the original
/// 0.1.x behaviour.
///
/// # Examples
///
/// ```
/// # #[allow(deprecated)]
/// # {
/// use adtobs::convert_utc_to_bs;
/// let nepali_date = convert_utc_to_bs("2023-11-29T12:00:00Z");
/// assert_eq!(nepali_date, "2080 Mangsir 13, Wednesday");
/// # }
/// ```
#[deprecated(
    since = "0.2.0",
    note = "use `NepaliDate::from_rfc3339(s)` instead"
)]
pub fn convert_utc_to_bs(utc_string: &str) -> String {
    match NepaliDate::from_rfc3339(utc_string) {
        Ok(d) => d.to_string(),
        Err(_) => String::from("Invalid date !"),
    }
}
