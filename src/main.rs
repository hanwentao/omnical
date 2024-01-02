fn main() {
    let year = 2024;
    println!("{year}");
    let num_days_of_month = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut weekday = 1;
    for month in 0..12 {
        println!("{}", month + 1);
        for _ in 0..weekday {
            print!("    ");
        }
        for day in 0..num_days_of_month[month] {
            print!("{:4}", day + 1);
            weekday = (weekday + 1) % 7;
            if weekday == 0 {
                println!();
            }
        }
        if weekday != 0 {
            println!();
        }
    }
}
