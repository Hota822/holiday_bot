use chrono::Duration;
use chrono::naive::NaiveDate;
use chrono::Local;


// pub fn holidays_in_month() Vec<> {
pub fn holidays_in_month() -> Option<Vec<(NaiveDate, &'static str)>> {
    let mut vector = Vec::new();
    let next_month = next_month();
    loop {

        // let holidays = db.holiday.get
        // let holiday = holidays.pop()
        // if holiday.day < next_month {
            // vector.push((holiday.day, holiday.description))
        // } else {
            // break;
        // }
        break // if implemented this scope, delete this line
    };

    if vector.len() == 0 {
        None
    } else {
        Some(vector)
    }
}

pub fn next_month() -> NaiveDate {
    (Local::today() + Duration::days(30)).naive_local()
}

pub fn tomorrow() -> NaiveDate {
    (Local::today() + Duration::days(1)).naive_local()
}

pub fn update_next_holiday() {
    // let holiday = db.first.get()
    // db.insret public_holiday::
}