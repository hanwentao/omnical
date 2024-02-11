use strum::{Display, EnumCount, EnumString, FromRepr, VariantArray};

use crate::date::*;

#[cfg(test)]
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Year {
    year: i32,
}

impl Year {
    pub fn new(year: i32) -> Self {
        Self { year }
    }

    pub fn ordinal(&self) -> i32 {
        self.year
    }

    pub fn is_leap(&self) -> bool {
        self.year % 400 == 0 || (self.year % 4 == 0 && self.year % 100 != 0)
    }

    pub fn num_months(&self) -> u8 {
        12
    }

    pub fn num_days(&self) -> u16 {
        if self.is_leap() {
            366
        } else {
            365
        }
    }

    pub fn months(&self) -> Vec<Month> {
        MonthName::VARIANTS
            .iter()
            .map(|m| Month::new(self.year, *m))
            .collect()
    }
}

#[test]
fn test_year() {
    let year = Year::new(1985);
    assert_eq!(year.ordinal(), 1985);
    assert!(!year.is_leap());
    assert_eq!(year.num_months(), 12);
    assert_eq!(year.num_days(), 365);

    assert!(Year::new(2024).is_leap());
    assert!(Year::new(2000).is_leap());
    assert!(!Year::new(1900).is_leap());
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, EnumCount, VariantArray, Display, EnumString, FromRepr,
)]
#[repr(u8)]
pub enum MonthName {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

impl MonthName {
    pub fn ordinal(&self) -> u8 {
        *self as u8 + 1
    }

    pub fn from_ordinal(ord: u8) -> Option<Self> {
        if ord > 0 {
            Self::from_repr(ord - 1)
        } else {
            None
        }
    }
}

pub use MonthName::*;

#[test]
fn test_month_name() {
    assert_eq!(MonthName::COUNT, 12);
    assert_eq!(
        MonthName::VARIANTS,
        &[
            January, February, March, April, May, June, July, August, September, October, November,
            December,
        ]
    );

    let jan = MonthName::from_str("January").unwrap();
    assert_eq!(jan as u8, 0);
    assert_eq!(jan.ordinal(), 1);
    assert_eq!(jan.to_string(), "January");

    let dec = MonthName::from_repr(11).unwrap();
    assert_eq!(dec as u8, 11);
    assert_eq!(dec.ordinal(), 12);
    assert_eq!(dec.to_string(), "December");

    assert!(MonthName::from_str("Invalid").is_err());
    assert!(MonthName::from_repr(12).is_none());
    assert_eq!(MonthName::from_ordinal(9), Some(September));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Month {
    year: Year,
    month: MonthName,
}

impl Month {
    pub fn new(year: i32, month: MonthName) -> Self {
        Self {
            year: Year::new(year),
            month,
        }
    }

    pub fn from_ym(year: i32, month: u8) -> Option<Self> {
        MonthName::from_ordinal(month).map(|m| Self::new(year, m))
    }

    pub fn year(&self) -> Year {
        self.year
    }

    pub fn month(&self) -> MonthName {
        self.month
    }

    pub fn ordinal(&self) -> u8 {
        self.month.ordinal()
    }

    pub fn is_leap(&self) -> bool {
        self.month == February && self.year.is_leap()
    }

    pub fn num_days(&self) -> u8 {
        match self.month {
            January | March | May | July | August | October | December => 31,
            April | June | September | November => 30,
            February => {
                if self.year.is_leap() {
                    29
                } else {
                    28
                }
            }
        }
    }

    pub fn days(&self) -> Vec<Day> {
        (1..=self.num_days())
            .map(|d| Day::from_ymd(self.year.ordinal(), self.month.ordinal(), d).unwrap())
            .collect()
    }
}

#[test]
fn test_month() {
    let month = Month::new(1985, September);
    assert_eq!(month.year().ordinal(), 1985);
    assert_eq!(month.ordinal(), 9);
    assert!(!month.is_leap());
    assert_eq!(month.num_days(), 30);

    assert!(!Month::new(1985, February).is_leap());
    assert!(Month::new(2024, February).is_leap());
    assert!(Month::new(2000, February).is_leap());
    assert!(!Month::new(1900, February).is_leap());

    assert_eq!(Month::new(1986, January).num_days(), 31);

    assert!(Month::from_ym(2024, 0).is_none());
    assert!(Month::from_ym(2024, 13).is_none());
    assert_eq!(Month::from_ym(2024, 2), Some(Month::new(2024, February)));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Day {
    month: Month,
    day: u8,
}

impl Day {
    pub fn from_ymd(year: i32, month: u8, day: u8) -> Option<Self> {
        Month::from_ym(year, month).and_then(|ym| {
            if day >= 1 && day <= ym.num_days() {
                Some(Self {
                    month: ym,
                    day: day - 1,
                })
            } else {
                None
            }
        })
    }

    pub fn from_date(date: Date) -> Self {
        let (y, m, d) = astro::time::date_frm_julian_day(date.julian_date()).unwrap();
        Self::from_ymd(y as i32, m, d as u8).unwrap()
    }

    pub fn ordinal(&self) -> u8 {
        self.day + 1
    }

    pub fn year(&self) -> Year {
        self.month.year
    }

    pub fn month(&self) -> Month {
        self.month
    }

    pub fn as_date(&self) -> Date {
        let day_of_month = astro::time::DayOfMonth {
            day: self.ordinal(),
            hr: 0,
            min: 0,
            sec: 0.0,
            time_zone: 0.0,
        };
        let astro_date = astro::time::Date {
            year: self.year().ordinal() as i16,
            month: self.month.ordinal(),
            decimal_day: astro::time::decimal_day(&day_of_month),
            cal_type: astro::time::CalType::Gregorian,
        };
        Date::from_jd(astro::time::julian_day(&astro_date))
    }

    pub fn weekday(&self) -> Weekday {
        let date = self.as_date();
        date.weekday()
    }
}

#[test]
fn test_day() {
    let day = Day::from_ymd(1985, 9, 15).unwrap();
    assert_eq!(day.weekday(), Sunday);

    assert_eq!(Day::from_ymd(1582, 10, 15).unwrap().weekday(), Friday);
    assert_eq!(Day::from_ymd(2024, 2, 11).unwrap().weekday(), Sunday);

    assert!(Day::from_ymd(2024, 2, 29).is_some());
    assert!(Day::from_ymd(2022, 2, 29).is_none());
}
