//! A library aims to support multiple calendars
//!
//! Provides a generic `Date` struct that represents a date by Julian day, and
//! multiple calendar systems that can be converted to and from `Date`.

pub mod astronomy;
pub mod calendar;
pub mod chinese;
pub mod date;
pub mod gregorian;

pub use astronomy::*;
pub use calendar::*;
pub use chinese::{
    Branch, Day as ChineseDay, Month as ChineseMonth, Stem, StemBranch, Year as ChineseYear,
};
pub use date::*;
pub use gregorian::{
    Day as GregorianDay, Month as GregorianMonth, MonthName, MonthName::*, Year as GregorianYear,
};
