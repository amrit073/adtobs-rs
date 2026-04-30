use crate::dump::{LEAST_AD, LEAST_BS, MAX_AD, MAX_BS, NEPALI_YEARS_AND_DAYS_IN_MONTHS};
use chrono::{DateTime, Datelike, Duration, NaiveDate, Weekday};
use std::error::Error;
use std::fmt;
use std::str::FromStr;

const GREGORIAN_MONTH_DAYS: [u32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
const GREGORIAN_LEAP_MONTH_DAYS: [u32; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

/// Months of the Bikram Sambat calendar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum Month {
    Baisakh = 1,
    Jesth = 2,
    Asar = 3,
    Srawan = 4,
    Bhadra = 5,
    Aaswin = 6,
    Kartik = 7,
    Mangsir = 8,
    Paush = 9,
    Magh = 10,
    Falgun = 11,
    Chaitra = 12,
}

impl Month {
    /// Returns the English transliteration of the month name.
    pub const fn name(self) -> &'static str {
        match self {
            Month::Baisakh => "Baisakh",
            Month::Jesth => "Jesth",
            Month::Asar => "Asar",
            Month::Srawan => "Srawan",
            Month::Bhadra => "Bhadra",
            Month::Aaswin => "Aaswin",
            Month::Kartik => "Kartik",
            Month::Mangsir => "Mangsir",
            Month::Paush => "Paush",
            Month::Magh => "Magh",
            Month::Falgun => "Falgun",
            Month::Chaitra => "Chaitra",
        }
    }

    /// Returns the 1-based month number (Baisakh = 1, …, Chaitra = 12).
    pub const fn number(self) -> u8 {
        self as u8
    }

    /// Builds a `Month` from its 1-based number, or `None` if `n` is out of range.
    pub const fn from_number(n: u8) -> Option<Self> {
        match n {
            1 => Some(Month::Baisakh),
            2 => Some(Month::Jesth),
            3 => Some(Month::Asar),
            4 => Some(Month::Srawan),
            5 => Some(Month::Bhadra),
            6 => Some(Month::Aaswin),
            7 => Some(Month::Kartik),
            8 => Some(Month::Mangsir),
            9 => Some(Month::Paush),
            10 => Some(Month::Magh),
            11 => Some(Month::Falgun),
            12 => Some(Month::Chaitra),
            _ => None,
        }
    }
}

impl fmt::Display for Month {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// Errors produced when constructing a [`NepaliDate`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NepaliDateError {
    /// The year is outside the supported lookup table.
    OutOfRange { year: i32, min: i32, max: i32 },
    /// The Gregorian (year, month, day) tuple is not a real calendar date.
    InvalidGregorian { year: i32, month: u32, day: u32 },
    /// The Bikram Sambat (year, month, day) tuple is not a real calendar date.
    InvalidBs { year: i32, month: u32, day: u32 },
    /// Failed to parse a date/time string.
    Parse(String),
}

impl fmt::Display for NepaliDateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NepaliDateError::OutOfRange { year, min, max } => write!(
                f,
                "year {year} is outside the supported range {min}..={max}"
            ),
            NepaliDateError::InvalidGregorian { year, month, day } => {
                write!(f, "invalid Gregorian date: {year}-{month:02}-{day:02}")
            }
            NepaliDateError::InvalidBs { year, month, day } => {
                write!(f, "invalid Bikram Sambat date: {year}-{month:02}-{day:02}")
            }
            NepaliDateError::Parse(msg) => write!(f, "parse error: {msg}"),
        }
    }
}

impl Error for NepaliDateError {}

/// A date in the Bikram Sambat (Nepali) calendar.
///
/// Construct one with [`NepaliDate::today`], [`NepaliDate::from_gregorian_ymd`],
/// [`NepaliDate::from_gregorian`], or [`NepaliDate::from_rfc3339`]. Use
/// [`Display`](fmt::Display) to render the canonical
/// `"<year> <Month> <day>, <Weekday>"` form.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NepaliDate {
    year: i32,
    month: Month,
    day: u8,
    weekday: Weekday,
}

impl NepaliDate {
    /// Returns the current date in Nepal Time (UTC+05:45).
    ///
    /// # Panics
    ///
    /// Panics if the system clock falls outside the supported lookup table
    /// (currently `1944..=2033` AD). This range easily covers the present day.
    pub fn today() -> Self {
        let offset = chrono::FixedOffset::east_opt(5 * 3600 + 45 * 60)
            .expect("UTC+05:45 is a valid offset");
        let today = chrono::Utc::now().with_timezone(&offset).date_naive();
        Self::from_gregorian(today).expect("current date is within supported range")
    }

    /// Converts a Gregorian `(year, month, day)` triple to a Bikram Sambat date.
    pub fn from_gregorian_ymd(
        year: i32,
        month: u32,
        day: u32,
    ) -> Result<Self, NepaliDateError> {
        let date = NaiveDate::from_ymd_opt(year, month, day)
            .ok_or(NepaliDateError::InvalidGregorian { year, month, day })?;
        Self::from_gregorian(date)
    }

    /// Converts a [`chrono::NaiveDate`] in the Gregorian calendar to BS.
    pub fn from_gregorian(date: NaiveDate) -> Result<Self, NepaliDateError> {
        let year = date.year();
        if !(LEAST_AD..=MAX_AD).contains(&year) {
            return Err(NepaliDateError::OutOfRange {
                year,
                min: LEAST_AD,
                max: MAX_AD,
            });
        }
        Ok(convert_gregorian(year, date.month(), date.day()))
    }

    /// Parses an RFC 3339 / ISO 8601 timestamp (e.g. `"2023-11-29T12:00:00Z"`)
    /// and converts the resulting instant — interpreted in Nepal Time — to BS.
    pub fn from_rfc3339(s: &str) -> Result<Self, NepaliDateError> {
        let parsed = DateTime::parse_from_rfc3339(s)
            .map_err(|e| NepaliDateError::Parse(e.to_string()))?;
        let offset = chrono::FixedOffset::east_opt(5 * 3600 + 45 * 60)
            .expect("UTC+05:45 is a valid offset");
        Self::from_gregorian(parsed.with_timezone(&offset).date_naive())
    }

    /// Constructs a `NepaliDate` from a Bikram Sambat `(year, month, day)`
    /// triple, validating against the BS month-length lookup table.
    pub fn from_bs_ymd(
        year: i32,
        month: u32,
        day: u32,
    ) -> Result<Self, NepaliDateError> {
        if !(LEAST_BS..=MAX_BS).contains(&year) {
            return Err(NepaliDateError::OutOfRange {
                year,
                min: LEAST_BS,
                max: MAX_BS,
            });
        }
        let m = Month::from_number(month as u8)
            .ok_or(NepaliDateError::InvalidBs { year, month, day })?;
        let row = &NEPALI_YEARS_AND_DAYS_IN_MONTHS[(year - LEAST_BS) as usize];
        let max_day = row[month as usize] as u32;
        if day < 1 || day > max_day {
            return Err(NepaliDateError::InvalidBs { year, month, day });
        }
        let ad = bs_to_gregorian_unchecked(year, month, day);
        Ok(NepaliDate {
            year,
            month: m,
            day: day as u8,
            weekday: ad.weekday(),
        })
    }

    /// Returns the Gregorian (English) calendar date corresponding to this
    /// Bikram Sambat date.
    pub fn to_gregorian(&self) -> NaiveDate {
        bs_to_gregorian_unchecked(self.year, self.month.number() as u32, self.day as u32)
    }

    /// The BS year.
    pub const fn year(&self) -> i32 {
        self.year
    }

    /// The BS month.
    pub const fn month(&self) -> Month {
        self.month
    }

    /// The day of the month, 1-based.
    pub const fn day(&self) -> u8 {
        self.day
    }

    /// The weekday this date falls on.
    pub const fn weekday(&self) -> Weekday {
        self.weekday
    }
}

impl fmt::Display for NepaliDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {}, {}",
            self.year,
            self.month,
            self.day,
            weekday_full_name(self.weekday)
        )
    }
}

impl TryFrom<NaiveDate> for NepaliDate {
    type Error = NepaliDateError;

    fn try_from(date: NaiveDate) -> Result<Self, Self::Error> {
        Self::from_gregorian(date)
    }
}

impl From<NepaliDate> for NaiveDate {
    fn from(date: NepaliDate) -> Self {
        date.to_gregorian()
    }
}

impl FromStr for NepaliDate {
    type Err = NepaliDateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_rfc3339(s)
    }
}

const fn weekday_full_name(w: Weekday) -> &'static str {
    match w {
        Weekday::Sun => "Sunday",
        Weekday::Mon => "Monday",
        Weekday::Tue => "Tuesday",
        Weekday::Wed => "Wednesday",
        Weekday::Thu => "Thursday",
        Weekday::Fri => "Friday",
        Weekday::Sat => "Saturday",
    }
}

const fn weekday_from_idx(idx: i32) -> Weekday {
    // The conversion loop below numbers days as Sun=1, Mon=2, …, Sat=7.
    match idx {
        1 => Weekday::Sun,
        2 => Weekday::Mon,
        3 => Weekday::Tue,
        4 => Weekday::Wed,
        5 => Weekday::Thu,
        6 => Weekday::Fri,
        _ => Weekday::Sat,
    }
}

fn is_leap_year(year: i32) -> bool {
    if year % 100 == 0 {
        year % 400 == 0
    } else {
        year % 4 == 0
    }
}

fn convert_gregorian(yy: i32, mm: u32, dd: u32) -> NepaliDate {
    // Algorithm preserved verbatim from the original implementation: count the
    // number of Gregorian days elapsed since 1944-01-01, then walk forward the
    // same number of days through the BS month-length table starting from
    // 17 Paush 2000 BS.
    let mut total_e_days: i32 = 0;
    let mut day_idx: i32 = 6;

    for i in 0..(yy - LEAST_AD) {
        let months = if is_leap_year(LEAST_AD + i) {
            &GREGORIAN_LEAP_MONTH_DAYS
        } else {
            &GREGORIAN_MONTH_DAYS
        };
        for &d in months {
            total_e_days += d as i32;
        }
    }

    let cur_year_months = if is_leap_year(yy) {
        &GREGORIAN_LEAP_MONTH_DAYS
    } else {
        &GREGORIAN_MONTH_DAYS
    };
    for i in 0..(mm - 1) {
        total_e_days += cur_year_months[i as usize] as i32;
    }
    total_e_days += dd as i32;

    let mut i: usize = 0;
    let mut j: usize = 9; // def_nmm = 9 (Paush column)
    let mut total_n_days: i32 = 16; // def_ndd = 17 - 1
    let mut m: i32 = 9;
    let mut y: i32 = LEAST_BS;

    while total_e_days != 0 {
        let a = NEPALI_YEARS_AND_DAYS_IN_MONTHS[i][j];
        total_n_days += 1;
        day_idx += 1;
        if total_n_days > a {
            m += 1;
            total_n_days = 1;
            j += 1;
        }
        if day_idx > 7 {
            day_idx = 1;
        }
        if m > 12 {
            y += 1;
            m = 1;
        }
        if j > 12 {
            j = 1;
            i += 1;
        }
        total_e_days -= 1;
    }

    NepaliDate {
        year: y,
        month: Month::from_number(m as u8).expect("month index is 1..=12"),
        day: total_n_days as u8,
        weekday: weekday_from_idx(day_idx),
    }
}

/// Caller is expected to have validated `(y, m, d)` against the BS table.
fn bs_to_gregorian_unchecked(y: i32, m: u32, d: u32) -> NaiveDate {
    // BS 2000 Paush 17 ↔ AD 1944-01-01. Compute the day-offset between the
    // target BS date and that anchor by walking the BS month-length table,
    // then add it to 1944-01-01.
    let target = bs_day_index(y, m, d);
    let baseline = bs_day_index(LEAST_BS, 9, 17);
    let delta = target - baseline;
    NaiveDate::from_ymd_opt(LEAST_AD, 1, 1)
        .expect("1944-01-01 is a valid Gregorian date")
        + Duration::days(delta)
}

/// Number of days from BS `LEAST_BS`-Baisakh-1 (inclusive) up to and
/// including `(y, m, d)`. Assumes inputs are within the table.
fn bs_day_index(y: i32, m: u32, d: u32) -> i64 {
    let mut total: i64 = 0;
    for yy in LEAST_BS..y {
        let row = &NEPALI_YEARS_AND_DAYS_IN_MONTHS[(yy - LEAST_BS) as usize];
        total += row[1..=12].iter().map(|&x| x as i64).sum::<i64>();
    }
    let row = &NEPALI_YEARS_AND_DAYS_IN_MONTHS[(y - LEAST_BS) as usize];
    total += row[1..m as usize].iter().map(|&x| x as i64).sum::<i64>();
    total += d as i64;
    total
}
