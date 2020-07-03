use crate::public_holidays::*;
use crate::models::holiday;

pub fn reset_db() {
    let title = vec![
        "new_years_day",
        "adult_day",
        "national_foundation_day",
        "emperor_birthday",
        "vernal_equinox_day",
        "showa_day",
        "constitution_day",
        "green_day",
        "childrens_day",
        "sea_day",
        "mountain_day",
        "respect_for_the_aged_day",
        "autumn_equinox_day",
        "sports_day",
        "culture_day",
        "labor_thanksgiving_day",
    ];
    for f in functions {
        
    }
}

// private---------------------------------------------------------------------------

fn function_list<T>() -> (
    Box<T>,
    Box<T>,
    Box<T>,
    Box<T>,
    Box<T>,
    Box<T>,
    Box<T>,
    Box<T>,
    Box<T>,
    Box<T>,
    Box<T>,
    Box<T>,
    Box<T>,
    Box<T>,
    Box<T>,
    Box<T>,
)
where
    T: Fn() -> chrono::naive::NaiveDate,
{
    let functions = (
        Box::new(new_years_day),
        Box::new(adult_day),
        Box::new(national_foundation_day),
        Box::new(emperor_birthday),
        Box::new(vernal_equinox_day),
        Box::new(showa_day),
        Box::new(constitution_day),
        Box::new(green_day),
        Box::new(childrens_day),
        Box::new(sea_day),
        Box::new(mountain_day),
        Box::new(respect_for_the_aged_day),
        Box::new(autumn_equinox_day),
        Box::new(sports_day),
        Box::new(culture_day),
        Box::new(labor_thanksgiving_day),
    );
    functions
}