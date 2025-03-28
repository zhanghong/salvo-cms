use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};

/// 将给定的Unix时间戳转换为本地时间的NaiveDateTime。
///
/// # 参数
/// * `seconds` - 一个i64整数，表示自1970年1月1日00:00:00 UTC以来的秒数。
///
/// # 返回值
/// 返回一个NaiveDateTime对象，表示本地时间。
pub fn from_timestamp(seconds: i64) -> NaiveDateTime {
    let time = DateTime::from_timestamp(seconds, 0).unwrap();
    time.naive_local()
}

/// 将给定的NaiveDateTime转换为Unix时间戳。
///
/// # 参数
/// * `date` - 一个NaiveDateTime对象。
///
/// # 返回值
/// 返回一个i64整数，表示自1970年1月1日00:00:00 UTC以来的秒数。
pub fn to_timestamp(date: NaiveDateTime) -> i64 {
    date.and_utc().timestamp()
}

/// 获取当前时间作为NaiveDateTime对象。
///
/// # 返回值
/// 返回一个NaiveDateTime对象，表示当前的本地时间。
pub fn current_time() -> NaiveDateTime {
    Utc::now().naive_local()
}

/// 获取当前时间作为Unix时间戳。
///
/// # 返回值
/// 返回一个i64整数，表示当前时间自1970年1月1日00:00:00 UTC以来的秒数。
pub fn current_timestamp() -> i64 {
    let date = current_time();
    to_timestamp(date)
}

/// 将给定的NaiveDateTime格式化为数据库时间字符串。
///
/// # 参数
/// * `time` - 一个指向NaiveDateTime对象的引用。
///
/// # 返回值
/// 返回一个String，格式化的数据库时间字符串，格式为"YYYY-MM-DD HH:MM:SS"。
pub fn to_db_time(time: &NaiveDateTime) -> String {
    time.format("%Y-%m-%d %H:%M:%S").to_string()
}

/// 将给定的NaiveDate格式化为数据库日期字符串。
///
/// # 参数
/// * `date` - 一个指向NaiveDate对象的引用。
///
/// # 返回值
/// 返回一个String，格式化的数据库日期字符串，格式为"YYYY-MM-DD"。
pub fn to_db_date(date: &NaiveDate) -> String {
    date.format("%Y-%m-%d").to_string()
}
