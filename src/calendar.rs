use crate::date::{Date, Weekday};

pub trait Year: std::fmt::Display {
    type Month: Month;
    type Day: Day;

    fn ord(&self) -> i32;
    fn succ(&self) -> Self;
    fn pred(&self) -> Self;

    fn num_months(&self) -> usize;
    fn month(&self, ord: u8) -> Option<Self::Month>;
    fn first_month(&self) -> Self::Month {
        self.month(1).unwrap()
    }
    fn last_month(&self) -> Self::Month {
        self.month(self.num_months() as u8).unwrap()
    }
    fn months(&self) -> Vec<Self::Month> {
        (1..=self.num_months() as u8)
            .filter_map(|i| self.month(i))
            .collect()
    }

    fn num_days(&self) -> usize {
        self.months().iter().map(|m| m.num_days()).sum()
    }
    fn day(&self, ord: u16) -> Option<Self::Day>;
    fn first_day(&self) -> Self::Day {
        self.day(1).unwrap()
    }
    fn last_day(&self) -> Self::Day {
        self.day(self.num_days() as u16).unwrap()
    }
    fn days(&self) -> Vec<Self::Day> {
        (1..=self.num_days() as u16)
            .filter_map(|i| self.day(i))
            .collect()
    }

    fn is_leap(&self) -> bool {
        false
    }
}

pub trait Month: std::fmt::Display {
    type Year: Year;
    type Day: Day;

    fn ord(&self) -> u8;
    fn succ(&self) -> Self;
    fn pred(&self) -> Self;

    fn year(&self) -> Self::Year;

    fn num_days(&self) -> usize;
    fn day(&self, ord: u8) -> Option<Self::Day>;
    fn first_day(&self) -> Self::Day {
        self.day(1).unwrap()
    }
    fn last_day(&self) -> Self::Day {
        self.day(self.num_days() as u8).unwrap()
    }
    fn days(&self) -> Vec<Self::Day> {
        (1..=self.num_days() as u8)
            .filter_map(|i| self.day(i))
            .collect()
    }

    fn is_leap(&self) -> bool {
        false
    }
}

pub trait Day: Clone + Copy + std::fmt::Display + Into<Date> {
    type Year: Year;
    type Month: Month;

    fn ord(&self) -> u8;
    fn ord_in_year(&self) -> u16 {
        (1..self.month().ord())
            .map(|m| self.year().month(m).unwrap().num_days() as u16)
            .sum::<u16>()
            + self.ord() as u16
            + 1
    }
    fn succ(&self) -> Self;
    fn pred(&self) -> Self;

    fn year(&self) -> Self::Year;
    fn month(&self) -> Self::Month;

    fn is_leap(&self) -> bool {
        false
    }
    fn weekday(&self) -> Weekday {
        let date: Date = (*self).into();
        date.weekday()
    }
}
