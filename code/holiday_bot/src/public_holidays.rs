// use chrono::naive::NaiveDate;
// use chrono::{Datelike, Weekday};
use chrono::Datelike;
use chrono::naive::NaiveDate;
use chrono::Local;

// 元日	1月1日
pub fn new_years_day() -> NaiveDate {
    constant_day(1, 1)
}

// 成人の日	1月の第2月曜日
pub fn adult_day() -> NaiveDate  {
    constant_monday(1, 8)
}

// 建国記念の日	政令で定める日
pub fn national_foundation_day() -> NaiveDate {
    constant_day(2, 11)
}

// 天皇誕生日 2月23日
pub fn emperor_birthday() -> NaiveDate {
    constant_day(2, 23)
}

// 春分の日	春分日
pub fn vernal_equinox_day() -> NaiveDate {
    equinox_day(3)
    // constant_day(3, 20)
}

// 昭和の日	4月29日
pub fn showa_day() -> NaiveDate{
    constant_day(4, 29)
}

// 憲法記念日 5月3日
pub fn constitution_day() -> NaiveDate {
    constant_day(5, 3)
}

// みどりの日 5月4日
pub fn green_day() -> NaiveDate {
    constant_day(5, 4)
}

// こどもの日 5月5日
pub fn childrens_day() -> NaiveDate {
    constant_day(5, 5)
}

// 海の日 7月の第3月曜日
pub fn sea_day() -> NaiveDate{
    constant_monday(7, 15)
}

// 山の日 8月11日
pub fn mountain_day() -> NaiveDate{
    constant_day(8, 11)
}

// 敬老の日	9月の第3月曜日
pub fn respect_for_the_aged_day() -> NaiveDate{
    constant_monday(9, 15)
}

// 秋分の日	秋分日
pub fn autumn_equinox_day() -> NaiveDate{
    equinox_day(9)
}

// スポーツの日	10月の第2月曜日
pub fn sports_day() -> NaiveDate {
    constant_monday(10, 15)
}

// 文化の日	11月3日
pub fn culture_day() -> NaiveDate{
    constant_day(11, 3)
}

// 勤労感謝の日	11月23日
pub fn labor_thanksgiving_day()-> NaiveDate {
    constant_day(11, 23)
}

fn next_year() -> i32 {
    Local::today().year() + 1
}

fn constant_day(month: u32, day: u32) -> NaiveDate {
    NaiveDate::from_ymd(next_year(), month, day)

}

fn constant_monday(month: u32, day: u32) -> NaiveDate {
    // day: fastest possible monday
    let year = next_year();
    let diff = NaiveDate::from_ymd(year, month, day).weekday().num_days_from_monday();
    let day = if diff == 0 {
        8
    } else {
        8 + 7 - diff
    };
    NaiveDate::from_ymd(year, 1, day)
}

fn equinox_day(month: u32) -> NaiveDate {
    let year = next_year();
    let reminder = year % 4;
    let mut day;
    let min_reminder =
        if month == 3 {
            day = 20;
            match year {
                2000..=2023 => 2,
                2024..=2055 => 3,
                2056..=2091 => 4,
                _ => 0,
            }
        } else {
            day = 22;
            match year {
                2012..=2043 => 1,
                2044..=2075 => 2,
                2076..=2107 => 3,
                _ => 0,
            }
        };
    if min_reminder <= reminder {
        day += 1;
    }
    NaiveDate::from_ymd(year, month, day)
}