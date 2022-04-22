#![allow(warnings, unused)]
use chrono::{Month, Weekday};

pub fn weekday_to_index(day: Weekday) -> u32 {
    match day {
        Weekday::Sun => 1,
        Weekday::Mon => 2,
        Weekday::Tue => 3,
        Weekday::Wed => 4,
        Weekday::Thu => 5,
        Weekday::Fri => 6,
        Weekday::Sat => 7,
    }
}
pub fn index_to_weekday(day: u32) -> Weekday {
    match day {
        1 => Weekday::Sun,
        2 => Weekday::Mon,
        3 => Weekday::Tue,
        4 => Weekday::Wed,
        5 => Weekday::Thu,
        6 => Weekday::Fri,
        7 => Weekday::Sat,
        _ => {
            panic!();
        }
    }
}

pub fn month_length(month: u32, leap_year: bool) -> u32 {
    match month {
        1 => 31,
        2 => {
            if leap_year {
                29
            } else {
                28
            }
        }
        3 => 31,
        4 => 30,
        5 => 31,
        6 => 30,
        7 => 31,
        8 => 31,
        9 => 30,
        10 => 31,
        11 => 30,
        12 => 31,
        _ => {
            panic!();
        }
    }
}

pub fn leap_year(year: i32) -> bool {
    let four = year % 4 == 0;
    let hundred = year % 100 == 0;
    let four_hundred = year % 400 == 0;
    if !four {
        return false;
    }
    if hundred && !four_hundred {
        return false;
    }
    return true;
}
