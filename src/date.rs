use crate::dump;
use chrono::{DateTime, Datelike, NaiveDate, NaiveDateTime, TimeZone, Utc};
use chrono_tz::Asia::Kathmandu;

trait HasDate {
    fn get_year(&self) -> i32;
    fn get_month(&self) -> u32;
    fn get_day(&self) -> u32;
}

fn is_leap_year(year: i32) -> bool {
    if year % 100 == 0 {
        return year % 400 == 0;
    } else {
        return year % 4 == 0;
    }
}

fn check_if_date_is_in_range(year: i32, month: u32, day: u32) -> bool {
    if year < 1944 || year > 2033 {
        return false;
    }
    if month < 1 || month > 12 {
        return false;
    }

    !(day < 1 || day > 31)
}

fn get_english_month(month: i32) -> String {
    let english_month = match month {
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
    };
    english_month
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

fn convert_english_date_to_nepali<T: HasDate>(date: &T) -> String {
    let yy = date.get_year();
    let mm = date.get_month();
    let dd = date.get_day();
    if !check_if_date_is_in_range(yy, mm, dd) {
        return String::from("Invalid date !");
    }

    let month = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let leap_year_months = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    let def_eyy = 1944;
    let def_nyy = 2000;
    let def_nmm = 9;
    let def_ndd = 17 - 1;
    let mut total_e_days = 0;

    let mut day = 7 - 1;
    let mut i;
    let mut j;

    for i in 0..(yy - def_eyy) {
        if is_leap_year(def_eyy + i) {
            for j in 0..12 {
                total_e_days += leap_year_months[j];
            }
        } else {
            for j in 0..12 {
                total_e_days += month[j];
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
    let mut y = def_nyy;
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
        "{} {} {}, {}\n",
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

pub fn get_todays_np_date() -> String {
    let today = Utc::now().with_timezone(&Kathmandu);
    convert_english_date_to_nepali(&today)
}

pub fn convert_ad_to_bs(year: i32, month: u32, day: u32) -> String {
    let date = NaiveDate::from_ymd_opt(year, month, day)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();
    convert_english_date_to_nepali(&date)
}

pub fn convert_utc_to_bs(utc_string: &str) -> String {
    let date = Utc::datetime_from_str(&Utc, utc_string, "%+")
        .unwrap()
        .with_timezone(&Kathmandu);
    convert_english_date_to_nepali(&date)
}
