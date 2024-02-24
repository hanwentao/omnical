use strum::{AsRefStr, EnumCount, EnumProperty, EnumString, FromRepr, VariantArray};

use crate::astronomy::*;
use crate::*;

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
    /// Monday, the first day of the week.
    #[strum(props(zh = "星期一", zh2 = "周一", zh1 = "一"))]
    Monday,
    /// Tuesday, the second day of the week.
    #[strum(props(zh = "星期二", zh2 = "周二", zh1 = "二"))]
    Tuesday,
    /// Wednesday, the third day of the week.
    #[strum(props(zh = "星期三", zh2 = "周三", zh1 = "三"))]
    Wednesday,
    /// Thursday, the fourth day of the week.
    #[strum(props(zh = "星期四", zh2 = "周四", zh1 = "四"))]
    Thursday,
    /// Friday, the fifth day of the week.
    #[strum(props(zh = "星期五", zh2 = "周五", zh1 = "五"))]
    Friday,
    /// Saturday, the sixth day of the week.
    #[strum(props(zh = "星期六", zh2 = "周六", zh1 = "六"))]
    Saturday,
    /// Sunday, the seventh day of the week.
    #[strum(props(zh = "星期日", zh2 = "周日", zh1 = "日"))]
    Sunday,
}

pub use Weekday::*;

impl Weekday {
    /// The ordinal of the variant.
    pub const fn ord(&self) -> u8 {
        *self as u8 + 1
    }

    /// Create a variant from its ordinal.
    pub const fn from_ord(ord: u8) -> Option<Self> {
        Self::from_repr((ord as isize - 1) as usize)
    }

    /// The first variant.
    pub const fn first() -> Self {
        Monday
    }

    /// The last variant.
    pub const fn last() -> Self {
        Sunday
    }

    /// The next variant.
    pub const fn succ(&self) -> Self {
        *ignore_none(&Self::from_repr(
            (*self as isize + 1).rem_euclid(Self::COUNT as isize) as usize,
        ))
    }

    /// The previous variant.
    pub fn pred(&self) -> Self {
        *ignore_none(&Self::from_repr(
            (*self as isize - 1).rem_euclid(Self::COUNT as isize) as usize,
        ))
    }

    /// Chinese name of the variant.
    ///
    /// # Arguments
    ///
    /// * `length` - The length of the Chinese name, see examples below.
    ///
    /// # Examples
    ///
    /// ```
    /// # use omnical::*;
    /// assert_eq!(Weekday::Monday.chinese(1), "一");
    /// assert_eq!(Weekday::Monday.chinese(2), "周一");
    /// assert_eq!(Weekday::Monday.chinese(3), "星期一");
    /// assert_eq!(Weekday::Monday.chinese(0), "星期一");
    /// ```
    pub fn chinese(&self, length: usize) -> &str {
        match length {
            1 => self.get_str("zh1").unwrap(),
            2 => self.get_str("zh2").unwrap(),
            _ => self.get_str("zh").unwrap(),
        }
    }
}

impl std::fmt::Display for Weekday {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "{}", self.chinese(f.width().unwrap_or(0)))
        } else if let Some(width) = f.width() {
            if self.as_ref().len() > width {
                write!(f, "{}", &self.as_ref()[..width])
            } else {
                f.pad(self.as_ref())
            }
        } else {
            write!(f, "{}", self.as_ref())
        }
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

    assert_eq!(format!("{:#}", sun), "星期日");
    assert_eq!(format!("{:#2}", sun), "周日");
    assert_eq!(format!("{:#1}", sun), "日");
}

/// A generic date type using Julian day number (JDN) as its internal representation.
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
