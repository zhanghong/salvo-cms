use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};

pub fn from_timestamp(seconds: i64) -> NaiveDateTime {
    let time = DateTime::from_timestamp(seconds, 0).unwrap();
    time.naive_local()
}

pub fn to_timestamp(date: NaiveDateTime) -> i64 {
    date.and_utc().timestamp()
}

pub fn current_time() -> NaiveDateTime {
    Utc::now().naive_local()
}

pub fn current_timestamp() -> i64 {
    let date = current_time();
    to_timestamp(date)
}

pub fn to_db_time(time: &NaiveDateTime) -> String {
    time.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn to_db_date(date: &NaiveDate) -> String {
    date.format("%Y-%m-%d").to_string()
}
