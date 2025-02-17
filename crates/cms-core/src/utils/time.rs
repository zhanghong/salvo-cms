use chrono::{Local, NaiveDateTime};

pub fn current_time() -> NaiveDateTime {
    Local::now().naive_local()
}
