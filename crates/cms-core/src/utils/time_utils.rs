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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Datelike, NaiveDate, NaiveTime, TimeZone, Timelike, Utc};

    // Helper function to create a DateTime<Utc> from year, month, day, etc.
    fn ymdhms(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
    ) -> DateTime<Utc> {
        let date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
        let time = NaiveTime::from_hms_opt(hour, minute, second).unwrap();
        Utc.from_local_datetime(&date.and_time(time)).unwrap()
    }

    #[test]
    fn test_from_timestamp() {
        // Test with timestamp 0 (1970-01-01 00:00:00 UTC)
        let dt = from_timestamp(0);
        assert_eq!(dt.year(), 1970);
        assert_eq!(dt.month(), 1);
        assert_eq!(dt.day(), 1);

        // Test with a specific date (2023-04-05 12:30:45 UTC)
        let timestamp = ymdhms(2023, 4, 5, 12, 30, 45).timestamp();
        let dt = from_timestamp(timestamp);

        assert_eq!(dt.year(), 2023);
        assert_eq!(dt.month(), 4);
        assert_eq!(dt.day(), 5);
        assert_eq!(dt.hour(), 12);
        assert_eq!(dt.minute(), 30);
        assert_eq!(dt.second(), 45);
    }

    #[test]
    fn test_to_timestamp() {
        // Test with 1970-01-01 00:00:00
        let dt = NaiveDate::from_ymd_opt(1970, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        assert_eq!(to_timestamp(dt), 0);

        // Test with 2023-04-05 12:30:45
        let dt = NaiveDate::from_ymd_opt(2023, 4, 5)
            .unwrap()
            .and_hms_opt(12, 30, 45)
            .unwrap();
        let expected = ymdhms(2023, 4, 5, 12, 30, 45).timestamp();
        assert_eq!(to_timestamp(dt), expected);
    }

    #[test]
    fn test_current_time_and_timestamp() {
        let now = current_time();
        let now_ts = current_timestamp();
        let utc_now = Utc::now().naive_utc();

        // The difference between local and UTC time should be consistent
        let diff = now.signed_duration_since(utc_now);
        assert!(diff.num_seconds().abs() < 24 * 3600); // Should be within +/- 24 hours

        // The two ways to get current timestamp should be close
        let direct_ts = Utc::now().timestamp();
        assert!((now_ts - direct_ts).abs() < 2); // Allow 2 seconds difference
    }

    #[test]
    fn test_to_db_time() {
        // Test with various times
        let dt1 = NaiveDate::from_ymd_opt(2023, 4, 5)
            .unwrap()
            .and_hms_opt(12, 30, 45)
            .unwrap();
        assert_eq!(to_db_time(&dt1), "2023-04-05 12:30:45");

        let dt2 = NaiveDate::from_ymd_opt(1999, 12, 31)
            .unwrap()
            .and_hms_opt(23, 59, 59)
            .unwrap();
        assert_eq!(to_db_time(&dt2), "1999-12-31 23:59:59");

        let dt3 = NaiveDate::from_ymd_opt(0, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        assert_eq!(to_db_time(&dt3), "0000-01-01 00:00:00");
    }

    #[test]
    fn test_to_db_date() {
        // Test with various dates
        let d1 = NaiveDate::from_ymd_opt(2023, 4, 5).unwrap();
        assert_eq!(to_db_date(&d1), "2023-04-05");

        let d2 = NaiveDate::from_ymd_opt(1999, 12, 31).unwrap();
        assert_eq!(to_db_date(&d2), "1999-12-31");

        let d3 = NaiveDate::from_ymd_opt(0, 1, 1).unwrap();
        assert_eq!(to_db_date(&d3), "0000-01-01");
    }
}
