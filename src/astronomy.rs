use strum::{Display, EnumCount, EnumProperty, EnumString, FromRepr, VariantArray};
// use std::cmp::Ordering::*;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    EnumCount,
    VariantArray,
    Display,
    EnumString,
    FromRepr,
    EnumProperty,
)]
pub enum SolarTerm {
    #[strum(props(zh = "冬至"))]
    WinterSolstice,
    #[strum(props(zh = "小寒"))]
    MinorCold,
    #[strum(props(zh = "大寒"))]
    MajorCold,
    #[strum(props(zh = "立春"))]
    BeginningOfSpring,
    #[strum(props(zh = "雨水"))]
    RainWater,
    #[strum(props(zh = "惊蛰"))]
    AwakeningOfInsects,
    #[strum(props(zh = "春分"))]
    SpringEquinox,
    #[strum(props(zh = "清明"))]
    PureBrightness,
    #[strum(props(zh = "谷雨"))]
    GrainRain,
    #[strum(props(zh = "立夏"))]
    BeginningOfSummer,
    #[strum(props(zh = "小满"))]
    GrainBuds,
    #[strum(props(zh = "芒种"))]
    GrainInEar,
    #[strum(props(zh = "夏至"))]
    SummerSolstice,
    #[strum(props(zh = "小暑"))]
    MinorHeat,
    #[strum(props(zh = "大暑"))]
    MajorHeat,
    #[strum(props(zh = "立秋"))]
    BeginningOfAutumn,
    #[strum(props(zh = "处暑"))]
    EndOfHeat,
    #[strum(props(zh = "白露"))]
    WhiteDew,
    #[strum(props(zh = "秋分"))]
    AutumnEquinox,
    #[strum(props(zh = "寒露"))]
    ColdDew,
    #[strum(props(zh = "霜降"))]
    FrostsDescent,
    #[strum(props(zh = "立冬"))]
    BeginningOfWinter,
    #[strum(props(zh = "小雪"))]
    MinorSnow,
    #[strum(props(zh = "大雪"))]
    MajorSnow,
}

pub use SolarTerm::*;

impl SolarTerm {
    pub fn ord(&self) -> u8 {
        *self as u8 + 1
    }

    pub fn from_ord(ord: u8) -> Option<Self> {
        Self::from_repr((ord as i8 - 1) as usize)
    }

    pub fn is_mid_term(&self) -> bool {
        *self as i8 % 2 == 0
    }

    pub fn succ(&self) -> Self {
        Self::from_repr((*self as i8 + 1).rem_euclid(Self::COUNT as i8) as usize).unwrap()
    }

    pub fn pred(&self) -> Self {
        Self::from_repr((*self as i8 - 1).rem_euclid(Self::COUNT as i8) as usize).unwrap()
    }

    pub fn degrees(&self) -> f64 {
        (*self as i8 + (270 / 15) as i8).rem_euclid(Self::COUNT as i8) as f64 * 15.0
    }

    pub fn from_degree_range(begin_deg: f64, end_deg: f64) -> Option<Self> {
        let begin_ord = -(-begin_deg).div_euclid(15.0) as i64;
        let end_ord = -(-end_deg).div_euclid(15.0) as i64;
        if begin_ord < end_ord {
            Self::from_repr((begin_ord - 270 / 15).rem_euclid(Self::COUNT as i64) as usize)
        } else {
            None
        }
    }

    pub fn chinese(&self) -> &str {
        self.get_str("zh").unwrap()
    }
}

#[test]
fn test_solar_term() {
    assert!(WinterSolstice.is_mid_term());
    assert!(!PureBrightness.is_mid_term());

    assert_eq!(WinterSolstice.pred(), MajorSnow);
    assert_eq!(MajorSnow.succ(), WinterSolstice);
    assert_eq!(WinterSolstice.succ(), MinorCold);
    assert_eq!(MinorCold.pred(), WinterSolstice);

    assert_eq!(WinterSolstice.degrees(), 270.0);
    assert_eq!(SpringEquinox.degrees(), 0.0);
    assert_eq!(PureBrightness.degrees(), 15.0);

    assert_eq!(SolarTerm::from_degree_range(0.0, 1.0), Some(SpringEquinox));
    assert_eq!(
        SolarTerm::from_degree_range(269.0, 286.0),
        Some(WinterSolstice)
    );
    assert_eq!(
        SolarTerm::from_degree_range(-1.0, 361.0),
        Some(SpringEquinox)
    );
    assert_eq!(SolarTerm::from_degree_range(0.0, 1.0), Some(SpringEquinox));
    assert_eq!(SolarTerm::from_degree_range(271.0, 285.0), None);
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
    EnumString,
    FromRepr,
    EnumProperty,
)]
pub enum LunarPhase {
    #[strum(props(zh = "新月", emoji = "🌑"))]
    NewMoon,
    #[strum(props(zh = "眉月", emoji = "🌒"))]
    WaxingCrescent,
    #[strum(props(zh = "上弦月", emoji = "🌓"))]
    FirstQuarter,
    #[strum(props(zh = "上凸月", emoji = "🌔"))]
    WaxingGibbous,
    #[strum(props(zh = "满月", emoji = "🌕"))]
    FullMoon,
    #[strum(props(zh = "下凸月", emoji = "🌖"))]
    WaningGibbous,
    #[strum(props(zh = "下弦月", emoji = "🌗"))]
    LastQuarter,
    #[strum(props(zh = "残月", emoji = "🌘"))]
    WaningCrescent,
}

pub use LunarPhase::*;

impl LunarPhase {
    pub fn succ(&self) -> Self {
        Self::from_repr((*self as i8 + 1).rem_euclid(Self::COUNT as i8) as usize).unwrap()
    }

    pub fn pred(&self) -> Self {
        Self::from_repr((*self as i8 - 1).rem_euclid(Self::COUNT as i8) as usize).unwrap()
    }

    pub fn degrees(&self) -> f64 {
        (*self as i8 as f64) * 45.0
    }

    pub fn from_degree_range(begin_deg: f64, end_deg: f64) -> Self {
        let begin_ord = -(-begin_deg).div_euclid(90.0) as i64;
        let end_ord = -(-end_deg).div_euclid(90.0) as i64;
        if begin_ord < end_ord {
            Self::from_repr((begin_ord * 2).rem_euclid(Self::COUNT as i64) as usize).unwrap()
        } else {
            Self::from_repr((begin_ord * 2 - 1).rem_euclid(Self::COUNT as i64) as usize).unwrap()
        }
    }

    pub fn chinese(&self) -> &str {
        self.get_str("zh").unwrap()
    }

    pub fn emoji(&self) -> &str {
        self.get_str("emoji").unwrap()
    }
}

#[test]
fn test_lunar_phase() {
    assert_eq!(NewMoon.pred(), WaningCrescent);
    assert_eq!(WaningCrescent.succ(), NewMoon);
    assert_eq!(NewMoon.succ(), WaxingCrescent);
    assert_eq!(WaxingCrescent.pred(), NewMoon);

    assert_eq!(NewMoon.degrees(), 0.0);
    assert_eq!(FullMoon.degrees(), 180.0);
    assert_eq!(WaningGibbous.degrees(), 225.0);

    assert_eq!(LunarPhase::from_degree_range(-1.0, 1.0), NewMoon);
    assert_eq!(LunarPhase::from_degree_range(1.0, 89.0), WaxingCrescent);
    assert_eq!(LunarPhase::from_degree_range(1.0, 179.0), FirstQuarter);
    assert_eq!(LunarPhase::from_degree_range(91.0, 179.0), WaxingGibbous);
    assert_eq!(LunarPhase::from_degree_range(179.0, 181.0), FullMoon);
    assert_eq!(LunarPhase::from_degree_range(181.0, 269.0), WaningGibbous);
    assert_eq!(LunarPhase::from_degree_range(181.0, 359.0), LastQuarter);
    assert_eq!(LunarPhase::from_degree_range(271.0, 359.0), WaningCrescent);
}

pub fn get_sun_ecl_long(jd: f64) -> f64 {
    let (ecl_pnt, _) = astro::sun::geocent_ecl_pos(jd);
    ecl_pnt.long.to_degrees()
}

pub fn get_moon_ecl_long(jd: f64) -> f64 {
    let (ecl_pnt, _) = astro::lunar::geocent_ecl_pos(jd);
    ecl_pnt.long.to_degrees()
}

pub fn get_moon_ecl_long_to_sun(jd: f64) -> f64 {
    let moon_ecl_long = get_moon_ecl_long(jd);
    let sun_ecl_long = get_sun_ecl_long(jd);
    (moon_ecl_long - sun_ecl_long).rem_euclid(360.0)
}

// fn solve(func: fn(f64) -> f64, left: f64, right: f64, eps: f64) -> f64 {
//     let mut left = left;
//     let mut right = right;
//     let mut mid = (left + right) / 2.0;
//     while right - left > eps {
//         let val = func(mid);
//         match val.partial_cmp(&0.0) {
//             Some(Less) => left = mid,
//             Some(Equal) => break,
//             Some(Greater) => right = mid,
//             _ => (),
//         }
//         mid = (left + right) / 2.0;
//     }
//     mid
// }

// #[test]
// fn test_solve() {
//     let r = solve(|x| x / 2.0 - 0.4, 0.0, 1.0, 1e-6);
//     assert!((r - 0.8).abs() < 1e-6);

//     let r = solve(|x| x * x - 0.36, 0.0, 5.0, 1e-6);
//     assert!((r - 0.6).abs() < 1e-6);
// }
