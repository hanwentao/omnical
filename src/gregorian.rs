/// Proleptic Gregorian calendar
use strum::{AsRefStr, Display, EnumCount, EnumString, FromRepr, VariantArray};

use crate::calendar;
use crate::calendar::{Day as _, Month as _, Year as _};
use crate::date::*;

#[cfg(test)]
use std::str::FromStr;

/// Converts a proleptic Gregorian date to a Julian day.
pub fn proleptic_gregorian_to_julian_day(y: i32, m: u8, d: f64) -> f64 {
    let (y, m) = if m > 2 { (y, m) } else { (y - 1, m + 12) };
    let a = y.div_euclid(100);
    let b = 2 - a + a.div_euclid(4);
    (365.25 * (y + 4716) as f64).floor() + (30.6001 * (m + 1) as f64).floor() + d + b as f64
        - 1524.5
}

/// Converts a Julian day to a proleptic Gregorian date.
pub fn julian_day_to_proleptic_gregorian(jd: f64) -> (i32, u8, f64) {
    let jd = jd + 0.5;
    let z = jd.trunc();
    let f = jd.fract();
    let alpha = (z - 1867216.25).div_euclid(36524.25);
    let a = z + 1.0 + alpha - alpha.div_euclid(4.0);
    let b = a + 1524.0;
    let c = (b - 122.1).div_euclid(365.25);
    let d = (365.25 * c).floor();
    let e = (b - d).div_euclid(30.6001);
    let dom = b - d - (30.6001 * e).floor() + f;
    let (y, m) = if e < 14.0 {
        (c - 4716.0, e - 1.0)
    } else {
        (c - 4715.0, e - 13.0)
    };
    (y as i32, m as u8, dom)
}

#[cfg(test)]
fn check_proleptic_gregorian_and_julian_day(y: i32, m: u8, d: f64, jd: f64) {
    assert_eq!(proleptic_gregorian_to_julian_day(y, m, d), jd);
    assert_eq!(julian_day_to_proleptic_gregorian(jd), (y, m, d));
}

#[test]
fn test_proleptic_gregorian_and_julian_day() {
    check_proleptic_gregorian_and_julian_day(2000, 1, 1.5, 2451545.0);
    check_proleptic_gregorian_and_julian_day(1999, 1, 1.0, 2451179.5);
    check_proleptic_gregorian_and_julian_day(1987, 1, 27.0, 2446822.5);
    check_proleptic_gregorian_and_julian_day(1987, 6, 19.5, 2446966.0);
    check_proleptic_gregorian_and_julian_day(1988, 1, 27.0, 2447187.5);
    check_proleptic_gregorian_and_julian_day(1988, 6, 19.5, 2447332.0);
    check_proleptic_gregorian_and_julian_day(1900, 1, 1.0, 2415020.5);
    check_proleptic_gregorian_and_julian_day(1600, 1, 1.0, 2305447.5);
    check_proleptic_gregorian_and_julian_day(1600, 12, 31.0, 2305812.5);
    check_proleptic_gregorian_and_julian_day(1582, 10, 4.0, 2299149.5);
    check_proleptic_gregorian_and_julian_day(1582, 10, 15.0, 2299160.5);
    check_proleptic_gregorian_and_julian_day(1, 1, 1.0, 1721425.5);
    check_proleptic_gregorian_and_julian_day(0, 1, 1.0, 1721059.5);
    check_proleptic_gregorian_and_julian_day(-4713, 11, 24.5, 0.0);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Year {
    year: i32,
}

impl Year {
    fn new(year: i32) -> Self {
        Self { year }
    }

    pub fn from_ord(ord: i32) -> Self {
        Self::new(ord)
    }

    pub fn from_y(y: i32) -> Self {
        Self::from_ord(y)
    }

    pub fn month_by_name(&self, month_name: MonthName) -> Month {
        Month::new(*self, month_name)
    }
}

impl calendar::Year for Year {
    type Month = Month;
    type Day = Day;

    fn ord(&self) -> i32 {
        self.year
    }

    fn succ(&self) -> Self {
        Self::new(self.year + 1)
    }

    fn pred(&self) -> Self {
        Self::new(self.year - 1)
    }

    fn num_months(&self) -> usize {
        12
    }

    fn month(&self, month_ord: u8) -> Option<Month> {
        MonthName::from_ord(month_ord).map(|mn| self.month_by_name(mn))
    }

    fn first_month(&self) -> Month {
        self.month_by_name(MonthName::first())
    }

    fn last_month(&self) -> Month {
        self.month_by_name(MonthName::last())
    }

    fn months(&self) -> Vec<Month> {
        MonthName::VARIANTS
            .iter()
            .map(|mn| self.month_by_name(*mn))
            .collect()
    }

    fn num_days(&self) -> usize {
        if self.is_leap() {
            366
        } else {
            365
        }
    }

    fn day(&self, ord: u16) -> Option<Day> {
        let mut ord = ord as usize;
        for name in MonthName::VARIANTS.iter() {
            let month = self.month_by_name(*name);
            if ord <= month.num_days() {
                return month.day(ord as u8);
            }
            ord -= month.num_days();
        }
        None
    }

    fn is_leap(&self) -> bool {
        self.year % 400 == 0 || (self.year % 4 == 0 && self.year % 100 != 0)
    }
}

#[test]
fn test_year() {
    let year = Year::from_y(1985);
    assert_eq!(year.ord(), 1985);
    assert!(!year.is_leap());
    assert_eq!(year.num_months(), 12);
    assert_eq!(year.num_days(), 365);

    assert!(Year::from_y(2024).is_leap());
    assert!(Year::from_y(2000).is_leap());
    assert!(!Year::from_y(1900).is_leap());

    assert_eq!(year.succ(), Year::from_y(1986));
    assert_eq!(year.pred(), Year::from_y(1984));

    assert_eq!(year.month(1).unwrap(), year.month_by_name(January));
    assert_eq!(year.day(1).unwrap(), Day::from_ymd(1985, 1, 1).unwrap());
    assert_eq!(year.day(365).unwrap(), Day::from_ymd(1985, 12, 31).unwrap());
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    EnumCount,
    VariantArray,
    Display,
    AsRefStr,
    EnumString,
    FromRepr,
)]
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

pub use MonthName::*;

impl MonthName {
    pub fn ord(&self) -> u8 {
        *self as u8 + 1
    }

    pub fn from_ord(ord: u8) -> Option<Self> {
        Self::from_repr((ord as isize - 1) as usize)
    }

    pub fn first() -> Self {
        January
    }

    pub fn last() -> Self {
        December
    }

    pub fn succ(&self) -> Option<Self> {
        Self::from_repr((*self as i8 + 1) as usize)
    }

    pub fn pred(&self) -> Option<Self> {
        Self::from_repr((*self as i8 - 1) as usize)
    }
}

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
    assert_eq!(jan.ord(), 1);
    assert_eq!(jan.to_string(), "January");

    let dec = MonthName::from_repr(11).unwrap();
    assert_eq!(dec as u8, 11);
    assert_eq!(dec.ord(), 12);
    assert_eq!(dec.to_string(), "December");

    assert!(MonthName::from_str("Invalid").is_err());
    assert!(MonthName::from_repr(12).is_none());
    assert_eq!(MonthName::from_ord(9), Some(September));

    assert_eq!(January.succ(), Some(February));
    assert_eq!(January.pred(), None);
    assert_eq!(December.pred(), Some(November));
    assert_eq!(December.succ(), None);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Month {
    year: Year,
    month: MonthName,
}

impl Month {
    fn new(year: Year, month: MonthName) -> Self {
        Self { year, month }
    }

    pub fn from_yn(year: i32, month_name: MonthName) -> Self {
        Year::new(year).month_by_name(month_name)
    }

    pub fn from_ym(year: i32, month: u8) -> Option<Self> {
        MonthName::from_ord(month).map(|mn| Self::from_yn(year, mn))
    }

    pub fn name(&self) -> MonthName {
        self.month
    }
}

impl calendar::Month for Month {
    type Year = Year;
    type Day = Day;

    fn ord(&self) -> u8 {
        self.month.ord()
    }

    fn succ(&self) -> Self {
        if let Some(next_month) = self.month.succ() {
            self.year.month_by_name(next_month)
        } else {
            self.year.succ().first_month()
        }
    }

    fn pred(&self) -> Self {
        if let Some(prev_month) = self.month.pred() {
            self.year.month_by_name(prev_month)
        } else {
            self.year.pred().last_month()
        }
    }

    fn year(&self) -> Year {
        self.year
    }

    fn num_days(&self) -> usize {
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

    fn day(&self, day_ord: u8) -> Option<Day> {
        if day_ord < 1 || day_ord > self.num_days() as u8 {
            return None;
        }
        Some(Day::new(*self, (day_ord as i8 - 1) as u8))
    }

    fn is_leap(&self) -> bool {
        match self.month {
            February => self.year.is_leap(),
            _ => false,
        }
    }
}

#[test]
fn test_month() {
    let month = Month::from_yn(1985, September);
    assert_eq!(month.year().ord(), 1985);
    assert_eq!(month.ord(), 9);
    assert!(!month.is_leap());
    assert_eq!(month.num_days(), 30);

    assert!(!Month::from_yn(1985, February).is_leap());
    assert!(Month::from_yn(2024, February).is_leap());
    assert!(Month::from_yn(2000, February).is_leap());
    assert!(!Month::from_yn(1900, February).is_leap());

    assert_eq!(Month::from_yn(1986, January).num_days(), 31);

    assert_eq!(Month::from_ym(2024, 0), None);
    assert_eq!(Month::from_ym(2024, 13), None);
    assert_eq!(
        Month::from_ym(2024, 2),
        Some(Month::from_yn(2024, February))
    );

    assert_eq!(month.succ(), Month::from_yn(1985, October));
    assert_eq!(month.pred(), Month::from_yn(1985, August));
    assert_eq!(
        Month::from_yn(2024, January).pred(),
        Month::from_yn(2023, December)
    );
    assert_eq!(
        Month::from_yn(2023, December).succ(),
        Month::from_yn(2024, January)
    );
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Day {
    month: Month,
    day: u8,
}

impl Day {
    fn new(month: Month, day: u8) -> Self {
        Self { month, day }
    }

    pub fn from_ynd(year: i32, month_name: MonthName, day: u8) -> Option<Self> {
        Month::from_yn(year, month_name).day(day)
    }

    pub fn from_ymd(year: i32, month: u8, day: u8) -> Option<Self> {
        Month::from_ym(year, month).and_then(|m| m.day(day))
    }

    pub fn from_date_with_tz(date: Date, tz: f64) -> Self {
        let (y, m, d) = julian_day_to_proleptic_gregorian(date.midnight_jd(tz));
        Self::from_ymd(y as i32, m, d as u8).unwrap()
    }
}

impl calendar::Day for Day {
    type Year = Year;
    type Month = Month;

    fn ord(&self) -> u8 {
        self.day + 1
    }

    fn succ(&self) -> Self {
        if self.day == self.month.num_days() as u8 - 1 {
            self.month.succ().first_day()
        } else {
            Self::new(self.month, self.day + 1)
        }
    }

    fn pred(&self) -> Self {
        if self.day == 0 {
            self.month.pred().last_day()
        } else {
            Self::new(self.month, self.day - 1)
        }
    }

    fn year(&self) -> Year {
        self.month.year
    }

    fn month(&self) -> Month {
        self.month
    }
}

impl From<Day> for Date {
    fn from(day: Day) -> Self {
        Date::from_jd(proleptic_gregorian_to_julian_day(
            day.year().ord(),
            day.month().ord(),
            day.ord() as f64,
        ))
    }
}

impl From<Date> for Day {
    fn from(date: Date) -> Self {
        Self::from_date_with_tz(date, 0.0)
    }
}

#[test]
fn test_day() {
    assert_eq!(Date::from(Day::from_ymd(-4713, 11, 24).unwrap()).jdn(), 0);
    assert_eq!(
        Date::from(Day::from_ymd(2000, 1, 1).unwrap()).jdn(),
        2451545
    );

    assert_eq!(Day::from_ymd(1582, 10, 15).unwrap().weekday(), Friday);
    assert_eq!(Day::from_ymd(1985, 9, 15).unwrap().weekday(), Sunday);
    assert_eq!(Day::from_ymd(2024, 2, 11).unwrap().weekday(), Sunday);

    assert_ne!(Day::from_ymd(2024, 2, 29), None);
    assert_eq!(Day::from_ymd(2022, 2, 29), None);

    let last_day_of_2023 = Day::from_ymd(2023, 12, 31).unwrap();
    let first_day_of_2024 = Day::from_ymd(2024, 1, 1).unwrap();
    assert_eq!(last_day_of_2023.succ(), first_day_of_2024);
    assert_eq!(first_day_of_2024.pred(), last_day_of_2023);
}

impl std::fmt::Display for Year {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "{}年", self.ord())
        } else if f.sign_minus() {
            write!(f, "{}", self.ord())
        } else {
            write!(f, "{:04}", self.ord())
        }
    }
}

impl std::fmt::Display for Month {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "{:#}{}月", self.year, self.ord())
        } else if f.sign_minus() {
            write!(f, "{} {:-}", self.month.as_ref(), self.year)
        } else {
            write!(f, "{:04}-{:02}", self.year, self.ord())
        }
    }
}

impl std::fmt::Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "{:#}{}日", self.month(), self.ord())
        } else if f.sign_minus() {
            write!(f, "{} {:-}", self.ord(), self.month)
        } else {
            write!(f, "{}-{:02}", self.month(), self.ord())
        }
    }
}
