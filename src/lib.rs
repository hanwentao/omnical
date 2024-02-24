//! A library aims to support multiple calendars.
//!
//! Omnical provides a generic [`Date`] struct that represents a date by
//! [Julian day number](https://en.wikipedia.org/wiki/Julian_day), and multiple
//! calendar systems that can be converted to and from [`Date`].
//!
//! Currently supported calendars:
//!
//! * [`GregorianCalendar`]: [(Proleptic) Gregorian calendar](https://en.wikipedia.org/wiki/Proleptic_Gregorian_calendar)
//! * [`ChineseCalendar`]: [Chinese calendar](https://en.wikipedia.org/wiki/Chinese_calendar)

pub mod astronomy;
pub mod calendar;
pub mod chinese;
pub mod date;
pub mod gregorian;

pub use astronomy::{LunarPhase, LunarPhase::*, SolarTerm, SolarTerm::*};
pub use calendar::{Calendar, Day, Month, Year};
pub use chinese::{
    Branch, Calendar as ChineseCalendar, Day as ChineseDay, Month as ChineseMonth, Stem,
    StemBranch, Year as ChineseYear,
};
pub use date::{Date, Weekday, Weekday::*};
pub use gregorian::{
    Calendar as GregorianCalendar, Day as GregorianDay, Month as GregorianMonth, MonthName,
    MonthName::*, Year as GregorianYear,
};

/// Returns the current Unix time.
pub fn unix_time_now() -> u64 {
    let now = std::time::SystemTime::now();
    now.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()
}

/// Ignores the `None` variant of an `Option` and returns the inner value.
///
/// It is a work-around for unstable feature of `Option::unwrap` in const fn.
pub const fn ignore_none<T>(x: &Option<T>) -> &T {
    match x {
        Some(x) => x,
        None => unreachable!(),
    }
}
