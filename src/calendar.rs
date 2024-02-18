use crate::*;

pub trait Year: std::fmt::Display {
    type Month: Month;
    type Day: Day;

    fn months(&self) -> Vec<Self::Month>;
}

pub trait Month: std::fmt::Display {
    type Year: Year;
    type Day: Day;

    fn days(&self) -> Vec<Self::Day>;
}

pub trait Day: std::fmt::Display {
    type Year: Year;
    type Month: Month;

    fn as_date(&self) -> Date;
    fn weekday(&self) -> Weekday;
}
