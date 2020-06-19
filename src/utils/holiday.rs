use chrono::Duration;
use chrono::naive::NaiveDate;
use chrono::Local;
use crate::public_holidays::*;
use crate::models::Holiday;


// pub fn holidays_in_month() Vec<> {
pub fn holidays_in_month() -> Option<Vec<(NaiveDate, &'static str)>> {
    let mut vector = Vec::new();
    let next_month = next_month();
    loop {

        // let holidays = db.holiday.get as vector
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

pub fn update_next_holiday() -> Result<(), &'static str>{
    // let holiday = db.first.get()
    let holiday = Holiday::new("name", "day");
    let holiday = match holiday.name {
        "new_years_day"  => new_years_day(),
        "adult_day" => adult_day(),
        "national_foundation_day" => national_foundation_day(),
        "emperor_birthday" =>  emperor_birthday(),
        "vernal_equinox_day" => vernal_equinox_day(),
        "showa_day" => showa_day(),
        "constitution_day" => constitution_day(),
        "green_day" => green_day(),
        "childrens_day" => childrens_day(),
        "sea_day" => sea_day(),
        "mountain_day" => mountain_day(),
        "respect_for_the_aged_day" => respect_for_the_aged_day(),
        "autumn_equinox_day" => autumn_equinox_day(),
        "sports_day" => sports_day(),
        "culture_day" => culture_day(),
        "labor_thanksgiving_day" => labor_thanksgiving_day(),
        _ => return Err("Failure"),
    };
    // db.delete.first 
    // db.insert public_holiday(holiday)
    Ok(())
}