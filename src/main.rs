use chrono::prelude::*;
use std::env;
use std::process::{ExitCode, Termination};

const MONTHS: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];
const WEEKDAYS: [&str; 7] = [
    "Sunday",
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
];

fn main() -> impl Termination {
    // Parse command line arguments.
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: {} [<year>]", args[0]);
        return ExitCode::FAILURE;
    }

    let year: i32 = if args.len() == 2 {
        args[1].parse().expect("Invalid year")
    } else {
        let now = Local::now();
        now.year()
    };

    println!("{year}");
    let num_days_of_month = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut weekday = 1;
    for month in 0..12 {
        println!("{:^28}", MONTHS[month]);
        for weekday in &WEEKDAYS {
            print!("{:>4}", &weekday[..3]);
        }
        println!();
        for _ in 0..weekday {
            print!("{:>4}", "");
        }
        for day in 0..num_days_of_month[month] {
            print!("{:>4}", day + 1);
            weekday = (weekday + 1) % 7;
            if weekday == 0 {
                println!();
            }
        }
        if weekday != 0 {
            println!();
        }
    }

    ExitCode::SUCCESS
}
