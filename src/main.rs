use chrono::Datelike;
use strum::VariantArray;

use omnical::*;

fn main() {
    // Parse the command line arguments.
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 2 {
        println!("Usage: {} [<year>]", args[0]);
        std::process::exit(1);
    }
    let year: i32 = if args.len() == 2 {
        args[1].parse().expect("Invalid year")
    } else {
        chrono::Local::now().year()
    };

    // Print the Gregorian calendar for the given year.
    let year = GregorianYear::new(year);
    println!("{}", year.ordinal());
    for month in year.months() {
        println!("{:^28}", month.month().to_string());
        for weekday in Weekday::VARIANTS {
            print!("{:>4}", &weekday.to_string()[..3]);
        }
        println!();
        let days = month.days();
        for _ in 0..days.first().unwrap().weekday() as u8 {
            print!("{:>4}", "");
        }
        for day in &days {
            print!("{:>4}", day.ordinal());
            if day.weekday() == Sunday {
                println!();
            }
        }
        if days.last().unwrap().weekday() != Sunday {
            println!();
        }
    }
}
