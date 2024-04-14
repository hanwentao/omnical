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
    /// Query the information of a date.
    Query(QueryArgs),
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
struct OptionArgs {
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
    option: OptionArgs,
}

#[derive(Args, Debug)]
struct QueryArgs {
    /// The date to query.
    date: Option<String>,
    /// Query options.
    #[command(flatten)]
    option: OptionArgs,
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
            let today = Date::from_unix_time_with_tz(unix_time_now(), 8.0);
            let today = GregorianDay::from(today);
            (today.the_year().ord(), Some(today.the_month().ord()))
        }
        _ => unreachable!(),
    }
}

fn print_calendar(args: &PrintArgs) {
    let (y, m) = parse_range(&args.range);
    if let Some(m) = m {
        let month = GregorianCalendar::from_ym(y, m).unwrap();
        println!("{:^28}", format!("{:-}", month));
        print_month(month);
    } else {
        let year = GregorianCalendar::from_y(y).unwrap();
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
    let today: GregorianDay = Date::from_unix_time_with_tz(unix_time_now(), 8.0).into();
    for weekday in Weekday::VARIANTS {
        print!(" {:3}", weekday);
    }
    println!();
    let days = month.days();
    for _ in 0..month.first_day().weekday() as u8 {
        print!("    ");
    }
    for day in days {
        if day == today {
            print!("[{:>2}]", day.ord());
        } else {
            print!(" {:>2} ", day.ord());
        }
        if day.weekday() == Weekday::last() {
            println!();
        }
    }
    if month.last_day().weekday() != Weekday::last() {
        println!();
    }
}

fn list_month(month: GregorianMonth, options: &OptionArgs, chinese_day: &mut Option<ChineseDay>) {
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

fn list_year(year: GregorianYear, options: &OptionArgs, chinese_day: &mut Option<ChineseDay>) {
    for month in year.months() {
        list_month(month, options, chinese_day);
    }
}

fn list_dates(args: &ListArgs) {
    let (y, m) = parse_range(&args.range);
    if let Some(m) = m {
        let month = GregorianCalendar::from_ym(y, m).unwrap();
        list_month(month, &args.option, &mut None);
    } else {
        let year = GregorianCalendar::from_y(y).unwrap();
        list_year(year, &args.option, &mut None);
    }
}

fn query_date(args: &QueryArgs) {
    // TODO: Use parse function when it's available.
    let date = if let Some(date) = &args.date {
        let y: i32 = date[0..4].parse().unwrap();
        let m: u8 = date[4..6].parse().unwrap();
        let d: u8 = date[6..8].parse().unwrap();
        GregorianCalendar::from_ymd(y, m, d).unwrap().into()
    } else {
        Date::from_unix_time_with_tz(unix_time_now(), 8.0)
    };
    if args.option.chinese {
        println!("{}", ChineseDay::from(date));
    }
    if args.option.weekday {
        println!("{}", date.weekday());
    }
    if args.option.lunar_phase {
        println!("{}", date.lunar_phase(8.0).chinese());
    }
    if args.option.lunar_phase_emoji {
        println!("{}", date.lunar_phase(8.0).emoji());
    }
    if args.option.solar_term {
        if let Some(st) = date.solar_term(8.0) {
            println!("{}", st.chinese());
        }
    }
    if !args.option.chinese
        && !args.option.weekday
        && !args.option.lunar_phase
        && !args.option.lunar_phase_emoji
        && !args.option.solar_term
    {
        println!("{}", ChineseDay::from(date));
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Print(args)) => print_calendar(args),
        Some(Commands::List(args)) => list_dates(args),
        Some(Commands::Query(args)) => query_date(args),
        None => print_calendar(&cli.args),
    }
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}
