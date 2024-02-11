use strum::{Display, EnumCount, EnumString, FromRepr, VariantArray};

#[cfg(test)]
use std::str::FromStr;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, EnumCount, VariantArray, Display, EnumString, FromRepr,
)]
#[repr(u8)]
pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

pub use Weekday::*;

#[test]
fn test_weekday() {
    assert_eq!(Weekday::COUNT, 7);
    assert_eq!(
        Weekday::VARIANTS,
        &[Monday, Tuesday, Wednesday, Thursday, Friday, Saturday, Sunday]
    );

    let mon = Weekday::from_str("Monday").unwrap();
    assert_eq!(mon as u8, 0);
    assert_eq!(mon.to_string(), "Monday");

    let sun = Weekday::from_repr(6).unwrap();
    assert_eq!(sun as u8, 6);
    assert_eq!(sun.to_string(), "Sunday");

    assert!(Weekday::from_str("Badday").is_err());
    assert!(Weekday::from_repr(7).is_none());
}

#[derive(Debug, Clone, Copy)]
pub struct Date {
    julian_date: f64,
}

impl Date {
    pub fn from_jd(julian_date: f64) -> Self {
        Self { julian_date }
    }

    pub fn from_jdn(jdn: i64) -> Self {
        Self {
            julian_date: jdn as f64 - 0.5,
        }
    }

    pub fn julian_date(&self) -> f64 {
        self.julian_date
    }

    pub fn jdn(&self) -> i64 {
        (self.julian_date + 0.5) as i64
    }

    pub fn weekday(&self) -> Weekday {
        Weekday::from_repr(((self.julian_date + 0.5) as i64 % 7) as u8).unwrap()
    }
}

#[test]
fn test_date() {
    assert_eq!(Date::from_jd(2299159.5).weekday(), Thursday);
    assert_eq!(Date::from_jd(2299160.5).weekday(), Friday);
    assert_eq!(Date::from_jd(2446323.791667).weekday(), Sunday);
    assert_eq!(Date::from_jd(2460351.078669).weekday(), Saturday);

    assert_eq!(Date::from_jdn(2299160).weekday(), Thursday);
    assert_eq!(Date::from_jdn(2299161).weekday(), Friday);
    assert_eq!(Date::from_jdn(2446324).weekday(), Sunday);
    assert_eq!(Date::from_jdn(2460351).weekday(), Saturday);
}
