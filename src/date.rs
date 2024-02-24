use strum::{AsRefStr, EnumCount, EnumProperty, EnumString, FromRepr, VariantArray};

use crate::astronomy::*;

#[cfg(test)]
use std::str::FromStr;

/// The 7 days of the week.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    EnumCount,
    VariantArray,
    AsRefStr,
    EnumString,
    FromRepr,
    EnumProperty,
)]
pub enum Weekday {
    #[strum(props(zh = "星期一"))]
    Monday,
    #[strum(props(zh = "星期二"))]
    Tuesday,
    #[strum(props(zh = "星期三"))]
    Wednesday,
    #[strum(props(zh = "星期四"))]
    Thursday,
    #[strum(props(zh = "星期五"))]
    Friday,
    #[strum(props(zh = "星期六"))]
    Saturday,
    #[strum(props(zh = "星期日"))]
    Sunday,
}

pub use Weekday::*;

impl Weekday {
    /// The ordinal of the variant.
    pub fn ord(&self) -> u8 {
        *self as u8 + 1
    }

    /// Create a variant from its ordinal.
    pub fn from_ord(ord: u8) -> Option<Self> {
        Self::from_repr((ord as isize - 1) as usize)
    }

    /// The first variant.
    pub fn first() -> Self {
        Monday
    }

    /// The last variant.
    pub fn last() -> Self {
        Sunday
    }

    /// The next variant.
    pub fn succ(&self) -> Self {
        Self::from_repr((*self as i8 + 1).rem_euclid(Self::COUNT as i8) as usize).unwrap()
    }

    /// The previous variant.
    pub fn pred(&self) -> Self {
        Self::from_repr((*self as i8 - 1).rem_euclid(Self::COUNT as i8) as usize).unwrap()
    }

    /// Chinese name of the variant.
    pub fn chinese(&self) -> &str {
        self.get_str("zh").unwrap()
    }
}

#[test]
fn test_weekday() {
    assert_eq!(Weekday::COUNT, 7);
    assert_eq!(
        Weekday::VARIANTS,
        &[Monday, Tuesday, Wednesday, Thursday, Friday, Saturday, Sunday]
    );

    let mon = Weekday::from_str("Monday").unwrap();
    assert_eq!(mon as i8, 0);
    assert_eq!(mon.to_string(), "Monday");

    let sun = Weekday::from_repr(6).unwrap();
    assert_eq!(sun as i8, 6);
    assert_eq!(sun.to_string(), "Sunday");

    assert!(Weekday::from_str("Badday").is_err());
    assert!(Weekday::from_repr(7).is_none());

    assert!(Weekday::from_ord(0).is_none());
    assert_eq!(mon.ord(), 1);
    assert_eq!(sun.ord(), 7);

    assert_eq!(Monday.succ(), Tuesday);
    assert_eq!(Monday.pred(), Sunday);
    assert_eq!(Sunday.pred(), Saturday);
    assert_eq!(Sunday.succ(), Monday);
}

/// A generic date type using Julian Day Number (JDN) as its internal representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date {
    jdn: u32,
}

impl Date {
    fn new(jdn: u32) -> Self {
        Self { jdn }
    }

    pub fn from_jdn(jdn: u32) -> Self {
        Self::new(jdn)
    }

    pub fn jdn(&self) -> u32 {
        self.jdn
    }

    pub fn from_jd(jd: f64) -> Self {
        Self {
            jdn: (jd + 0.5).floor() as u32,
        }
    }

    pub fn from_jd_with_tz(jd: f64, tz: f64) -> Self {
        Self {
            jdn: (jd + 0.5 - tz / 24.0).floor() as u32,
        }
    }

    pub fn midnight_jd(&self, tz: f64) -> f64 {
        self.jdn as f64 - 0.5 - tz / 24.0
    }

    pub fn noon_jd(&self, tz: f64) -> f64 {
        self.jdn as f64 - tz / 24.0
    }

    pub fn jd(&self) -> f64 {
        self.midnight_jd(0.0)
    }

    pub fn from_unix_time_with_tz(unix_time: u64, tz: f64) -> Self {
        Self::from_jd(unix_time as f64 / 86400.0 + 2440587.5 - tz / 24.0)
    }

    pub fn from_unix_time(unix_time: u64) -> Self {
        Self::from_unix_time_with_tz(unix_time, 0.0)
    }

    pub fn succ(&self) -> Self {
        Self::new(self.jdn + 1)
    }

    pub fn pred(&self) -> Self {
        Self::new(self.jdn - 1)
    }

    pub fn weekday(&self) -> Weekday {
        Weekday::from_repr(self.jdn.rem_euclid(7) as usize).unwrap()
    }

    pub fn solar_term(&self, tz: f64) -> Option<SolarTerm> {
        let curr_sun_ecl_long = get_sun_ecl_long(self.midnight_jd(tz));
        let next_sun_ecl_long = get_sun_ecl_long(self.succ().midnight_jd(tz));
        let curr_sun_ecl_long = if next_sun_ecl_long < curr_sun_ecl_long {
            curr_sun_ecl_long - 360.0
        } else {
            curr_sun_ecl_long
        };
        SolarTerm::from_degree_range(curr_sun_ecl_long, next_sun_ecl_long)
    }

    pub fn lunar_phase(&self, tz: f64) -> LunarPhase {
        let curr_moon_ecl_long_to_sun = get_moon_ecl_long_to_sun(self.midnight_jd(tz));
        let next_moon_ecl_long_to_sun = get_moon_ecl_long_to_sun(self.succ().midnight_jd(tz));
        let curr_moon_ecl_long_to_sun = if next_moon_ecl_long_to_sun < curr_moon_ecl_long_to_sun {
            curr_moon_ecl_long_to_sun - 360.0
        } else {
            curr_moon_ecl_long_to_sun
        };
        LunarPhase::from_degree_range(curr_moon_ecl_long_to_sun, next_moon_ecl_long_to_sun)
    }
}

impl std::ops::AddAssign<i32> for Date {
    fn add_assign(&mut self, rhs: i32) {
        self.jdn = self.jdn.saturating_add_signed(rhs);
    }
}

impl std::ops::Add<i32> for Date {
    type Output = Self;

    fn add(self, rhs: i32) -> Self::Output {
        let mut result = self;
        result += rhs;
        result
    }
}

impl std::ops::Add<Date> for i32 {
    type Output = Date;

    fn add(self, rhs: Date) -> Self::Output {
        let mut result = rhs;
        result += self;
        result
    }
}

impl std::ops::Sub for Date {
    type Output = i32;

    fn sub(self, rhs: Self) -> Self::Output {
        self.jdn as i32 - rhs.jdn as i32
    }
}

#[test]
fn test_date() {
    let d1 = Date::from_jd(2299159.5);
    let d2 = Date::from_jd(2299160.5);
    assert_eq!(d1.weekday(), Thursday);
    assert_eq!(d2.weekday(), Friday);
    assert_eq!(Date::from_jd(2446323.791667).weekday(), Sunday);
    assert_eq!(Date::from_jd(2460351.078669).weekday(), Saturday);

    assert_eq!(Date::from_jdn(2299160).weekday(), Thursday);
    assert_eq!(Date::from_jdn(2299161).weekday(), Friday);
    assert_eq!(Date::from_jdn(2446324).weekday(), Sunday);
    assert_eq!(Date::from_jdn(2460351).weekday(), Saturday);

    assert_eq!(
        Date::from_jdn(2460301).solar_term(8.0),
        Some(WinterSolstice)
    );
    assert_eq!(Date::from_jdn(2460292).lunar_phase(8.0), NewMoon);

    assert_eq!(d2 - d1, 1);
}

pub fn now_in_unix_time() -> u64 {
    let now = std::time::SystemTime::now();
    now.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()
}

impl std::fmt::Display for Weekday {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "{}", self.chinese())
        } else if f.sign_minus() {
            write!(f, "{}", &self.as_ref()[..3])
        } else {
            write!(f, "{}", self.as_ref())
        }
    }
}
