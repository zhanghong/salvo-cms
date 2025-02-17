use chrono::{Local, NaiveDate, NaiveDateTime};

pub fn current_time() -> NaiveDateTime {
    Local::now().naive_local()
}

pub fn to_db_time(time: &NaiveDateTime) -> String {
    time.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn to_db_date(date: &NaiveDate) -> String {
    date.format("%Y-%m-%d").to_string()
}
