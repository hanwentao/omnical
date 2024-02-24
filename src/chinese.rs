use derivative::Derivative;
use strum::{Display, EnumCount, EnumProperty, EnumString, FromRepr, VariantArray};

use crate::astronomy::*;
use crate::calendar;
use crate::calendar::{Day as _, Month as _, Year as _};
use crate::date::*;
use crate::GregorianDay;

const BEIJING_TZ: f64 = 8.0;
const LEAP_NAMES: [&str; 2] = ["", "闰"];
const MONTH_NAMES: [&str; 12] = [
    "正月",
    "二月",
    "三月",
    "四月",
    "五月",
    "六月",
    "七月",
    "八月",
    "九月",
    "十月",
    "十一月",
    "十二月",
];
const DAY_NAMES: [&str; 30] = [
    "初一", "初二", "初三", "初四", "初五", "初六", "初七", "初八", "初九", "初十", "十一", "十二",
    "十三", "十四", "十五", "十六", "十七", "十八", "十九", "二十", "廿一", "廿二", "廿三", "廿四",
    "廿五", "廿六", "廿七", "廿八", "廿九", "三十",
];

fn get_winter_solstice(year: i16, tz: f64) -> Date {
    let mut d: Date = GregorianDay::from_ymd(year, 12, 21).unwrap().into();
    while d.solar_term(tz) != Some(WinterSolstice) {
        d = d.succ();
    }
    d
}

fn get_prev_new_moon(date: Date, tz: f64) -> Date {
    let mut d = date;
    while d.lunar_phase(tz) != NewMoon {
        d = d.pred();
    }
    d
}

fn calc_chinese_year_period_data(year: i16) -> (Date, Vec<u8>, Option<usize>) {
    let mut data = Vec::new();
    let last_ws = get_winter_solstice(year - 1, BEIJING_TZ);
    let next_ws_p1 = get_winter_solstice(year, BEIJING_TZ).succ();
    let nm_before_last_ws = get_prev_new_moon(last_ws, BEIJING_TZ);
    let mut d = nm_before_last_ws;
    let mut last_nm = None;
    let mut has_mt = false;
    while d != next_ws_p1 {
        let lp = d.lunar_phase(BEIJING_TZ);
        let st = d.solar_term(BEIJING_TZ);
        if lp == NewMoon || st.is_some() {
            if lp == NewMoon {
                if let Some(last_nm) = last_nm {
                    let num_days = d - last_nm;
                    data.push((num_days as u8, has_mt));
                }
                last_nm = Some(d);
                has_mt = false;
            }
            if let Some(st) = st {
                if st.is_mid_term() {
                    has_mt = true;
                }
            }
        }
        d = d.succ();
    }
    let is_leap_year = data.len() > 12;
    let leap_month = if is_leap_year {
        let mut i = 0;
        loop {
            if !data[i].1 {
                break Some(i);
            }
            i += 1;
        }
    } else {
        None
    };
    let data = data.into_iter().map(|(x, _)| x).collect();
    (nm_before_last_ws, data, leap_month)
}

fn calc_chinese_year_data(year: i16) -> (Date, [u8; 13], u8) {
    let (fd1, data1, lm1) = calc_chinese_year_period_data(year);
    let (_, data2, lm2) = calc_chinese_year_period_data(year + 1);
    let (off1, nlm1) = match lm1 {
        Some(lm1) => {
            if lm1 <= 2 {
                (3, None)
            } else {
                (2, Some(lm1 - 2))
            }
        }
        None => (2, None),
    };
    let (off2, nlm2) = match lm2 {
        Some(lm2) => {
            if lm2 <= 2 {
                (3, Some(lm2 + 10))
            } else {
                (2, None)
            }
        }
        None => (2, None),
    };
    let mut data = [&data1[off1..], &data2[..off2]].concat();
    if data.len() == 12 {
        data.push(0);
    }
    let num_days_of_months: [u8; 13] = data.try_into().unwrap();
    let nlm = nlm1.or(nlm2);
    let fd = fd1 + data1[..off1].iter().sum::<u8>() as i32;
    let leap_month = nlm.unwrap_or(13) as u8;
    (fd, num_days_of_months, leap_month)
}

#[test]
fn test_calc_chinese_year_data() {
    let result = calc_chinese_year_data(2014);
    assert_eq!(
        result,
        (
            Date::from_jdn(2456689),
            [29, 30, 29, 30, 29, 30, 29, 30, 30, 29, 30, 29, 30],
            9
        )
    );
    let result = calc_chinese_year_data(2023);
    assert_eq!(
        result,
        (
            Date::from_jdn(2459967),
            [29, 30, 29, 29, 30, 30, 29, 30, 30, 29, 30, 29, 30],
            2
        )
    );
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
pub enum Stem {
    #[strum(props(zh = "甲"))]
    Jia,
    #[strum(props(zh = "乙"))]
    Yi,
    #[strum(props(zh = "丙"))]
    Bing,
    #[strum(props(zh = "丁"))]
    Ding,
    #[strum(props(zh = "戊"))]
    Wu,
    #[strum(props(zh = "己"))]
    Ji,
    #[strum(props(zh = "庚"))]
    Geng,
    #[strum(props(zh = "辛"))]
    Xin,
    #[strum(props(zh = "壬"))]
    Ren,
    #[strum(props(zh = "癸"))]
    Gui,
}

impl Stem {
    pub fn ord(&self) -> i8 {
        *self as i8 + 1
    }

    pub fn from_ord(ord: i8) -> Option<Self> {
        Self::from_repr((ord - 1) as usize)
    }

    pub fn from_year(year: i16) -> Self {
        Self::from_repr((year - 4).rem_euclid(Self::COUNT as i16) as usize).unwrap()
    }

    pub fn chinese(&self) -> &str {
        self.get_str("zh").unwrap()
    }
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
pub enum Branch {
    #[strum(props(zh = "子"))]
    Zi,
    #[strum(props(zh = "丑"))]
    Chou,
    #[strum(props(zh = "寅"))]
    Yin,
    #[strum(props(zh = "卯"))]
    Mao,
    #[strum(props(zh = "辰"))]
    Chen,
    #[strum(props(zh = "巳"))]
    Si,
    #[strum(props(zh = "午"))]
    Wu,
    #[strum(props(zh = "未"))]
    Wei,
    #[strum(props(zh = "申"))]
    Shen,
    #[strum(props(zh = "酉"))]
    You,
    #[strum(props(zh = "戌"))]
    Xu,
    #[strum(props(zh = "亥"))]
    Hai,
}

impl Branch {
    pub fn ord(&self) -> i8 {
        *self as i8 + 1
    }

    pub fn from_ord(ord: i8) -> Option<Self> {
        Self::from_repr((ord - 1) as usize)
    }

    pub fn from_year(year: i16) -> Self {
        Self::from_repr((year - 4).rem_euclid(Self::COUNT as i16) as usize).unwrap()
    }

    pub fn chinese(&self) -> &str {
        self.get_str("zh").unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StemBranch {
    stem: Stem,
    branch: Branch,
}

impl StemBranch {
    pub fn new(stem: Stem, branch: Branch) -> Self {
        Self { stem, branch }
    }

    pub fn new_with_repr(repr: usize) -> Self {
        let s = repr % 10;
        let b = repr % 12;
        Self::new(Stem::from_repr(s).unwrap(), Branch::from_repr(b).unwrap())
    }

    pub fn ord(&self) -> usize {
        let m = self.stem as isize;
        let n = self.branch as isize;
        (m * 6 - n * 5).rem_euclid(60) as usize + 1
    }

    pub fn from_ord(ord: usize) -> Option<Self> {
        if !(1..=60).contains(&ord) {
            return None;
        }
        Some(Self::new_with_repr(ord - 1))
    }

    pub fn from_stem_branch(stem: Stem, branch: Branch) -> Option<Self> {
        if stem.ord() % 2 != branch.ord() % 2 {
            return None;
        }
        Some(Self::new(stem, branch))
    }

    pub fn from_year(year: i16) -> Self {
        Self::new(Stem::from_year(year), Branch::from_year(year))
    }
}

#[test]
fn test_stem_branch() {
    assert_eq!(
        StemBranch::from_ord(1),
        StemBranch::from_stem_branch(Stem::Jia, Branch::Zi)
    );
    assert_eq!(
        StemBranch::from_stem_branch(Stem::Jia, Branch::Zi)
            .unwrap()
            .ord(),
        1
    );
    assert_eq!(
        StemBranch::from_ord(2),
        StemBranch::from_stem_branch(Stem::Yi, Branch::Chou)
    );
    assert_eq!(
        StemBranch::from_stem_branch(Stem::Yi, Branch::Chou)
            .unwrap()
            .ord(),
        2
    );
    assert_eq!(
        StemBranch::from_ord(13),
        StemBranch::from_stem_branch(Stem::Bing, Branch::Zi)
    );
    assert_eq!(
        StemBranch::from_stem_branch(Stem::Bing, Branch::Zi)
            .unwrap()
            .ord(),
        13
    );
    assert_eq!(
        StemBranch::from_ord(41),
        StemBranch::from_stem_branch(Stem::Jia, Branch::Chen)
    );
    assert_eq!(
        StemBranch::from_stem_branch(Stem::Jia, Branch::Chen)
            .unwrap()
            .ord(),
        41
    );
    assert_eq!(
        StemBranch::from_ord(60),
        StemBranch::from_stem_branch(Stem::Gui, Branch::Hai)
    );
    assert_eq!(
        StemBranch::from_stem_branch(Stem::Gui, Branch::Hai)
            .unwrap()
            .ord(),
        60
    );
}

pub struct Calendar;

impl calendar::Calendar for Calendar {
    type Year = Year;
    type Month = Month;
    type Day = Day;

    fn from_y(year: i16) -> Self::Year {
        Year::from_y(year)
    }

    fn from_ymo(year: i16, month: u8) -> Option<Self::Month> {
        Month::from_ym(year, month)
    }

    fn from_ymdo(year: i16, month: u8, day: u8) -> Option<Self::Day> {
        Day::from_ymd(year, month, day)
    }
}

impl Calendar {
    pub fn from_ylmo(year: i16, leap: bool, month: u8) -> Option<Month> {
        Month::from_ylm(year, leap, month)
    }

    pub fn from_ylmdo(year: i16, leap: bool, month: u8, day: u8) -> Option<Day> {
        Day::from_ylmd(year, leap, month, day)
    }
}

#[derive(Debug, Clone, Copy, Derivative)]
#[derivative(PartialEq, Eq)]
pub struct Year {
    year: i16,
    #[derivative(PartialEq = "ignore")]
    first_day: Date,
    #[derivative(PartialEq = "ignore")]
    num_days_of_months: [u8; 13],
    #[derivative(PartialEq = "ignore")]
    leap_month: u8,
}

impl Year {
    fn new(year: i16) -> Self {
        let (first_day, num_days_of_months, leap_month) = calc_chinese_year_data(year);
        Self {
            year,
            first_day,
            num_days_of_months,
            leap_month,
        }
    }

    pub fn from_y(year: i16) -> Self {
        Self::new(year)
    }

    pub fn stem(&self) -> Stem {
        Stem::from_year(self.year)
    }

    pub fn branch(&self) -> Branch {
        Branch::from_year(self.year)
    }

    pub fn stem_branch(&self) -> StemBranch {
        StemBranch::from_year(self.year)
    }
}

impl calendar::Year<Calendar> for Year {
    fn ord(&self) -> i16 {
        self.year
    }

    fn succ(&self) -> Self {
        Self::new(self.year + 1)
    }

    fn pred(&self) -> Self {
        Self::new(self.year - 1)
    }

    fn num_months(&self) -> usize {
        if self.leap_month < 13 {
            13
        } else {
            12
        }
    }

    fn month(&self, ord: u8) -> Option<Month> {
        if ord >= 1 && ord <= self.num_months() as u8 {
            Some(Month::new(*self, ord - 1))
        } else {
            None
        }
    }

    fn is_leap(&self) -> bool {
        self.leap_month < 13
    }
}

#[test]
fn test_year() {
    let year = Year::from_y(2021);
    assert_eq!(year.ord(), 2021);
    assert_eq!(year.stem(), Stem::Xin);
    assert_eq!(year.branch(), Branch::Chou);
    assert_eq!(
        year.stem_branch(),
        StemBranch::from_stem_branch(Stem::Xin, Branch::Chou).unwrap()
    );
    assert!(!year.is_leap());
    assert!(Year::from_y(2023).is_leap());

    assert_eq!(year.day(1), Day::from_ymd(2021, 1, 1));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Month {
    year: Year,
    month: u8,
}

impl Month {
    fn new(year: Year, month: u8) -> Self {
        Self { year, month }
    }

    pub fn from_ym(year: i16, month: u8) -> Option<Self> {
        let year = Year::from_y(year);
        year.month(month)
    }

    pub fn from_ylm(year: i16, leap: bool, month: u8) -> Option<Self> {
        let year = Year::from_y(year);
        if leap && month != year.leap_month {
            None
        } else if month < year.leap_month || !leap && month == year.leap_month {
            year.month(month)
        } else {
            year.month(month + 1)
        }
    }
}

impl calendar::Month<Calendar> for Month {
    fn ord(&self) -> u8 {
        self.month + 1
    }

    fn succ(&self) -> Self {
        if self.month < self.year.num_months() as u8 - 1 {
            Self::new(self.year, self.month + 1)
        } else {
            self.year.succ().first_month()
        }
    }

    fn pred(&self) -> Self {
        if self.month > 0 {
            Self::new(self.year, self.month - 1)
        } else {
            self.year.pred().last_month()
        }
    }

    fn the_year(&self) -> Year {
        self.year
    }

    fn num_days(&self) -> usize {
        self.year.num_days_of_months[self.month as usize] as usize
    }

    fn day(&self, ord: u8) -> Option<Day> {
        if ord > 0 && ord <= self.num_days() as u8 {
            Some(Day::new(*self, ord - 1))
        } else {
            None
        }
    }

    fn is_leap(&self) -> bool {
        self.year.leap_month == self.month
    }
}

#[test]
fn test_month() {
    let year = Year::from_y(2023);
    assert_eq!(Month::from_ym(2023, 1).unwrap(), Month::new(year, 0));
    assert_eq!(Month::from_ym(2023, 2).unwrap(), Month::new(year, 1));
    assert_eq!(Month::from_ym(2023, 3).unwrap(), Month::new(year, 2));
    assert_eq!(
        Month::from_ylm(2023, false, 1).unwrap(),
        Month::new(year, 0)
    );
    assert_eq!(
        Month::from_ylm(2023, false, 2).unwrap(),
        Month::new(year, 1)
    );
    assert_eq!(
        Month::from_ylm(2023, false, 3).unwrap(),
        Month::new(year, 3)
    );
    assert_eq!(Month::from_ylm(2023, true, 1), None);
    assert_eq!(Month::from_ylm(2023, true, 2).unwrap(), Month::new(year, 2));
    assert_eq!(Month::from_ylm(2023, true, 3), None);
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

    pub fn from_ymd(year: i16, month: u8, day: u8) -> Option<Self> {
        Month::from_ym(year, month).and_then(|m| {
            if day > 0 && day <= m.num_days() as u8 {
                Some(Self::new(m, day - 1))
            } else {
                None
            }
        })
    }

    pub fn from_ylmd(year: i16, leap: bool, month: u8, day: u8) -> Option<Self> {
        Month::from_ylm(year, leap, month).and_then(|m| {
            if day > 0 && day <= m.num_days() as u8 {
                Some(Self::new(m, day - 1))
            } else {
                None
            }
        })
    }

    pub fn from_date_with_tz(date: Date, tz: f64) -> Self {
        let gd = GregorianDay::from_date_with_tz(date, tz);
        let cy = Year::from_y(gd.the_year().ord());
        let cd = cy.first_day();
        let cd_date = Date::from(cd);
        if date >= cd_date {
            cy.day((date - cd_date) as u16 + 1).unwrap()
        } else {
            let cy = cy.pred();
            let cd = cy.first_day();
            let cd_date = Date::from(cd);
            cy.day((date - cd_date) as u16 + 1).unwrap()
        }
    }

    pub fn stem_branch(&self) -> StemBranch {
        let date = Date::from(*self);
        let repr = (date.jdn() + 18).rem_euclid(60) as usize;
        StemBranch::new_with_repr(repr)
    }
}

impl calendar::Day<Calendar> for Day {
    fn ord(&self) -> u8 {
        self.day + 1
    }

    fn succ(&self) -> Self {
        if self.day < self.month.num_days() as u8 - 1 {
            Self::new(self.month, self.day + 1)
        } else {
            self.month.succ().first_day()
        }
    }

    fn pred(&self) -> Self {
        if self.day > 0 {
            Self::new(self.month, self.day - 1)
        } else {
            self.month.pred().last_day()
        }
    }

    fn the_year(&self) -> Year {
        self.month.year
    }

    fn the_month(&self) -> Month {
        self.month
    }
}

impl From<Day> for Date {
    fn from(day: Day) -> Self {
        day.the_year().first_day
            + (0..day.month.month)
                .map(|m| day.the_year().num_days_of_months[m as usize] as i32)
                .sum::<i32>()
            + day.day as i32
    }
}

impl From<Date> for Day {
    fn from(date: Date) -> Self {
        Self::from_date_with_tz(date, BEIJING_TZ)
    }
}

#[test]
fn test_day() {
    let day = Day::from_ymd(1949, 8, 10).unwrap();
    assert_eq!(
        day.stem_branch(),
        StemBranch::from_stem_branch(Stem::Jia, Branch::Zi).unwrap()
    );
}

impl std::fmt::Display for Year {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(
                f,
                "公元{}年农历{}{}年",
                self.year,
                self.stem().chinese(),
                self.branch().chinese()
            )
        } else {
            write!(f, "{}{}年", self.stem().chinese(), self.branch().chinese())
        }
    }
}

impl std::fmt::Display for Month {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let l = self.year.leap_month < 13 && self.month == self.year.leap_month;
        let m = if self.month < self.year.leap_month {
            self.month
        } else {
            self.month - 1
        };
        if f.alternate() {
            write!(
                f,
                "{:#}{}{}",
                self.year, LEAP_NAMES[l as usize], MONTH_NAMES[m as usize]
            )
        } else {
            write!(
                f,
                "{}{}{}",
                self.year, LEAP_NAMES[l as usize], MONTH_NAMES[m as usize]
            )
        }
    }
}

impl std::fmt::Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "{:#}{}", self.month, DAY_NAMES[self.day as usize])
        } else {
            write!(f, "{}{}", self.month, DAY_NAMES[self.day as usize])
        }
    }
}
