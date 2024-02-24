use crate::date::{Date, Weekday};

pub trait Calendar: Sized {
    type Year: Year<Self>;
    type Month: Month<Self>;
    type Day: Day<Self>;
}

pub trait Year<C: Calendar>: Sized + std::fmt::Display {
    fn ord(&self) -> i16;
    fn succ(&self) -> Self;
    fn pred(&self) -> Self;

    fn num_months(&self) -> usize;
    fn month(&self, ord: u8) -> Option<C::Month>;
    fn first_month(&self) -> C::Month {
        self.month(1).unwrap()
    }
    fn last_month(&self) -> C::Month {
        self.month(self.num_months() as u8).unwrap()
    }
    fn months(&self) -> impl Iterator<Item = C::Month> {
        (1..=self.num_months() as u8).filter_map(|i| self.month(i))
    }

    fn num_days(&self) -> usize {
        self.months().map(|m| m.num_days()).sum()
    }
    fn day(&self, ord: u16) -> Option<C::Day> {
        let mut ord = ord;
        for month in self.months() {
            let num_days = month.num_days() as u16;
            if ord <= num_days {
                return month.day(ord as u8);
            }
            ord -= num_days;
        }
        None
    }
    fn first_day(&self) -> C::Day {
        self.day(1).unwrap()
    }
    fn last_day(&self) -> C::Day {
        self.day(self.num_days() as u16).unwrap()
    }
    fn days(&self) -> impl Iterator<Item = C::Day> {
        (1..=self.num_days() as u16).filter_map(|i| self.day(i))
    }

    fn is_leap(&self) -> bool {
        false
    }
}

pub trait Month<C: Calendar>: Sized + std::fmt::Display {
    fn ord(&self) -> u8;
    fn succ(&self) -> Self;
    fn pred(&self) -> Self;

    fn the_year(&self) -> C::Year;

    fn num_days(&self) -> usize;
    fn day(&self, ord: u8) -> Option<C::Day>;
    fn first_day(&self) -> C::Day {
        self.day(1).unwrap()
    }
    fn last_day(&self) -> C::Day {
        self.day(self.num_days() as u8).unwrap()
    }
    fn days(&self) -> impl Iterator<Item = C::Day> {
        (1..=self.num_days() as u8).filter_map(|i| self.day(i))
    }

    fn is_leap(&self) -> bool {
        false
    }
}

pub trait Day<C: Calendar>: Sized + Clone + Copy + std::fmt::Display + Into<Date> {
    fn ord(&self) -> u8;
    fn ord_in_year(&self) -> u16 {
        (1..self.the_month().ord())
            .map(|m| self.the_year().month(m).unwrap().num_days() as u16)
            .sum::<u16>()
            + self.ord() as u16
            + 1
    }
    fn succ(&self) -> Self;
    fn pred(&self) -> Self;

    fn the_year(&self) -> C::Year;
    fn the_month(&self) -> C::Month;

    fn is_leap(&self) -> bool {
        false
    }
    fn weekday(&self) -> Weekday {
        let date: Date = (*self).into();
        date.weekday()
    }
}
