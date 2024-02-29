use chinese_lunisolar_calendar::{LunisolarDate, SolarDate};
use chrono::NaiveDate;
use nongli::ChineseDate;
use omnical::*;

#[test]
fn verify_chinese_calendar() {
    let mut v = vec![];
    for year in 1928..2057 {
        let year = GregorianCalendar::from_y(year).unwrap();
        let mut chinese_day = ChineseDay::from(Date::from(year.first_day()));
        for day in year.days() {
            let y = day.the_year().ord() as u16;
            let m = day.the_month().ord();
            let d = day.ord();

            let y1 = chinese_day.the_year().ord() as u16;
            let m1 = chinese_day.the_month().ord_no_leap();
            let l1 = chinese_day.the_month().is_leap();
            let d1 = chinese_day.ord();

            let solar_date = SolarDate::from_ymd(y, m, d).unwrap();
            let lunisolar_date = LunisolarDate::from_solar_date(solar_date).unwrap();

            let y2 = lunisolar_date.to_lunisolar_year().to_u16();
            let m2 = lunisolar_date.to_lunar_month().to_u8();
            let l2 = lunisolar_date.to_lunar_month().is_leap_month();
            let d2 = lunisolar_date.to_lunar_day().to_u8();

            let naive_date = NaiveDate::from_ymd_opt(y as i32, m as u32, d as u32).unwrap();
            let nongli_date = ChineseDate::from_gregorian(&naive_date).unwrap();
            let y3 = nongli_date.year();
            let m3 = nongli_date.month();
            let l3 = nongli_date.leap();
            let d3 = nongli_date.day();

            if (y1, m1, l1, d1) != (y2, m2, l2, d2) || (y1, m1, l1, d1) != (y3, m3, l3, d3) {
                v.push(((y1, m1, l1, d1), (y2, m2, l2, d2), (y3, m3, l3, d3)))
            }

            chinese_day = chinese_day.succ();
        }
    }
    assert_eq!(v, vec![]);
}
