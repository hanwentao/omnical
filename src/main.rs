use clap::{Args, Parser, Subcommand};
use strum::VariantArray as _;

use omnical::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(flatten)]
    args: PrintArgs,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Print a calendar.
    Print(PrintArgs),
    /// List the days of a calendar in details.
    List(ListArgs),
    // TODO: Convert a date from one calendar to another.
    // TODO: Query the information of a date.
}

#[derive(Args, Debug)]
struct RangeArgs {
    /// The year.
    year: Option<i32>,
    /// The month.
    month: Option<u8>,
}

#[derive(Args, Debug)]
struct PrintArgs {
    /// The range of the calendar to print.
    #[command(flatten)]
    range: RangeArgs,
}

#[derive(Args, Debug)]
struct ListOptionArgs {
    /// Display the date in Chinese calendar.
    #[arg(short, long)]
    chinese: bool,
    /// Display the weekday.
    #[arg(short, long)]
    weekday: bool,
    /// Display the lunar phase.
    #[arg(short, long)]
    lunar_phase: bool,
    /// Display the lunar phase emoji.
    #[arg(short = 'e', long)]
    lunar_phase_emoji: bool,
    /// Display the solar term if applicable.
    #[arg(short, long)]
    solar_term: bool,
}

#[derive(Args, Debug)]
struct ListArgs {
    /// The range of the calendar to list.
    #[command(flatten)]
    range: RangeArgs,
    /// List options.
    #[command(flatten)]
    option: ListOptionArgs,
}

fn parse_range(args: &RangeArgs) -> (i32, Option<u8>) {
    match args {
        RangeArgs {
            year: Some(y),
            month: Some(m),
        } => (*y, Some(*m)),
        RangeArgs {
            year: Some(y),
            month: None,
        } => (*y, None),
        RangeArgs {
            year: None,
            month: None,
        } => {
            let today = Date::from_unix_time_with_tz(now_in_unix_time(), 8.0);
            let today = GregorianDay::from(today);
            (today.year().ord(), Some(today.month().ord()))
        }
        _ => unreachable!(),
    }
}

fn print_calendar(args: &PrintArgs) {
    let (y, m) = parse_range(&args.range);
    if let Some(m) = m {
        let month = GregorianMonth::from_ym(y, m).unwrap();
        println!("{:^28}", format!("{:-}", month));
        print_month(month);
    } else {
        let year = GregorianYear::from_y(y);
        println!("{:^28}", format!("Year {}", year));
        print_year(year);
    }
}

fn print_year(year: GregorianYear) {
    for month in year.months() {
        println!("{:^28}", month.name());
        print_month(month);
    }
}

fn print_month(month: GregorianMonth) {
    for weekday in Weekday::VARIANTS {
        print!("{:>4}", &weekday.to_string()[..3]);
    }
    println!();
    let days = month.days();
    for _ in 0..days.first().unwrap().weekday() as u8 {
        print!("{:>4}", "");
    }
    for day in &days {
        print!("{:>4}", day.ord());
        if day.weekday() == Weekday::last() {
            println!();
        }
    }
    if days.last().unwrap().weekday() != Weekday::last() {
        println!();
    }
}

fn list_month<M: Month>(month: M, options: &ListOptionArgs, chinese_day: &mut Option<ChineseDay>) {
    for day in month.days() {
        let date: Date = day.into();
        print!("{:#}", day);
        if options.chinese {
            if chinese_day.is_none() {
                *chinese_day = Some(ChineseDay::from(date));
            } else {
                *chinese_day = Some(chinese_day.unwrap().succ());
            }
            print!(" {}", chinese_day.unwrap());
        }
        if options.weekday {
            print!(" {:#}", day.weekday());
        }
        if options.lunar_phase {
            print!(" {}", date.lunar_phase(8.0).chinese());
        }
        if options.lunar_phase_emoji {
            print!(" {}", date.lunar_phase(8.0).emoji());
        }
        if options.solar_term {
            if let Some(st) = date.solar_term(8.0) {
                print!(" {}", st.chinese());
            }
        }
        println!();
    }
}

fn list_year<Y: Year>(year: Y, options: &ListOptionArgs, chinese_day: &mut Option<ChineseDay>) {
    for month in year.months() {
        list_month(month, options, chinese_day);
    }
}

fn list_dates(args: &ListArgs) {
    let (y, m) = parse_range(&args.range);
    if let Some(m) = m {
        let month = GregorianMonth::from_ym(y, m).unwrap();
        list_month(month, &args.option, &mut None);
    } else {
        let year = GregorianYear::from_y(y);
        list_year(year, &args.option, &mut None);
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Print(args)) => print_calendar(args),
        Some(Commands::List(args)) => list_dates(args),
        None => print_calendar(&cli.args),
    }
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}
