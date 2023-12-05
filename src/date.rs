use crate::dump::{self, LEAST_AD, LEAST_BS, MAX_AD};
use chrono::{DateTime, Datelike, NaiveDateTime, TimeZone};

pub trait HasDate {
    fn get_year(&self) -> i32;
    fn get_month(&self) -> u32;
    fn get_day(&self) -> u32;
}

fn is_leap_year(year: i32) -> bool {
    if year % 100 == 0 {
        year % 400 == 0
    } else {
        year % 4 == 0
    }
}

fn check_if_date_is_in_range(year: i32, month: u32, day: u32) -> bool {
    if !(LEAST_AD..=MAX_AD).contains(&year) {
        return false;
    }
    if !(1..=12).contains(&month) {
        return false;
    }
    if !(1..=31).contains(&day) {
        return false;
    }

    true
}

fn get_english_month(month: i32) -> String {
    match month {
        1 => "Baisakh".to_string(),
        2 => "Jesth".to_string(),
        3 => "Asar".to_string(),
        4 => "Srawan".to_string(),
        5 => "Bhadra".to_string(),
        6 => "Aaswin".to_string(),
        7 => "Kartik".to_string(),
        8 => "Mangsir".to_string(),
        9 => "Paush".to_string(),
        10 => "Magh".to_string(),
        11 => "Falgun".to_string(),
        12 => "Chaitra".to_string(),
        _ => "".to_string(),
    }
}

fn get_english_day_of_week_in_string(day: i32) -> String {
    match day {
        1 => "Sunday".to_string(),
        2 => "Monday".to_string(),
        3 => "Tuesday".to_string(),
        4 => "Wednesday".to_string(),
        5 => "Thursday".to_string(),
        6 => "Friday".to_string(),
        7 => "Saturday".to_string(),
        _ => "".to_string(),
    }
}

pub fn convert_english_date_to_nepali<T: HasDate>(date: &T) -> String {
    let yy = date.get_year();
    let mm = date.get_month();
    let dd = date.get_day();
    if !check_if_date_is_in_range(yy, mm, dd) {
        return String::from("Invalid date !");
    }

    let month = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let leap_year_months = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    let def_nmm = 9;
    let def_ndd = 17 - 1;
    let mut total_e_days = 0;

    let mut day = 7 - 1;
    let mut i;
    let mut j;

    for i in 0..(yy - LEAST_AD) {
        if is_leap_year(LEAST_AD + i) {
            for days in &leap_year_months {
                total_e_days += days;
            }
        } else {
            for days in &month {
                total_e_days += days;
            }
        }
    }

    for i in 0..(mm - 1) {
        if is_leap_year(yy) {
            total_e_days += leap_year_months[i as usize];
        } else {
            total_e_days += month[i as usize];
        }
    }

    total_e_days += dd;

    i = 0;
    j = def_nmm;
    let mut total_n_days = def_ndd;
    let mut m = def_nmm;
    let mut y = LEAST_BS;
    let mut a: i32;

    while total_e_days != 0 {
        a = dump::NEPALI_YEARS_AND_DAYS_IN_MONTHS[i as usize][j as usize];
        total_n_days += 1;
        day += 1;
        if total_n_days > a {
            m += 1;
            total_n_days = 1;
            j += 1;
        }
        if day > 7 {
            day = 1;
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

    let date_string = format!(
        "{} {} {}, {}",
        y,
        get_english_month(m),
        total_n_days,
        get_english_day_of_week_in_string(day)
    );
    date_string
}

impl<Tz: TimeZone> HasDate for DateTime<Tz> {
    fn get_year(&self) -> i32 {
        self.year()
    }

    fn get_month(&self) -> u32 {
        self.month()
    }

    fn get_day(&self) -> u32 {
        self.day()
    }
}

impl HasDate for NaiveDateTime {
    fn get_year(&self) -> i32 {
        self.year()
    }

    fn get_month(&self) -> u32 {
        self.month()
    }

    fn get_day(&self) -> u32 {
        self.day()
    }
}
