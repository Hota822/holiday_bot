use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::schema::public_holiday;

#[derive(Serialize, Deserialize, Queryable)]
pub struct PublicHoliday {
    pub id: i64,
    pub date: NaiveDate,
    pub title: String,
}

#[derive(Deserialize, Insertable)]
#[table_name = "public_holiday"]
pub struct InsertablePublicHoliday {
    pub date: NaiveDate,
    pub title: String,
}
