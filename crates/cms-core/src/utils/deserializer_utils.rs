use chrono::NaiveDateTime;
use num_traits::{Bounded, NumCast, Zero};
use serde::{Deserialize, Deserializer};
use serde_json::Value;
use std::str::FromStr;
use tracing::{debug, warn};

use super::parameter_utils;

// ------------------------------------------------------------------------
// 字符串类型处理方法
// ------------------------------------------------------------------------

/// 将字符串反序列化为Option<String>，去除前后空格
///
/// # Arguments
///
/// * `deserializer`: D类型的反序列化器
///
/// # Returns
///
/// * `Result<Option<String>, D::Error>`: 去除前后空格的字符串，如果字符串为空则返回None
pub fn string_to_option_trimmed<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;

    match value {
        // 处理字符串类型，去掉前后空格
        Value::String(s) => {
            let trimmed = s.trim();
            if trimmed.is_empty() {
                Ok(None)
            } else {
                Ok(Some(trimmed.to_string()))
            }
        }
        // 其他类型返回 None
        _ => Ok(None),
    }
}

/// 将字符串反序列化为Option<Vec<String>>，分割字符串为向量
///
/// # Arguments
///
/// * `deserializer`: D类型的反序列化器
///
/// # Returns
///
/// * `Result<Option<Vec<String>>, D::Error>`: 分割后的字符串向量，如果为空则返回None
pub fn string_to_option_string_vec<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    debug!("Deserializing string to vector");
    let value = Value::deserialize(deserializer)?;

    match value {
        Value::String(s) => {
            debug!("Input string: {:?}", s);
            let vec: Vec<String> = s
                .split(",")
                .map(|part| part.trim().to_string())
                .filter(|part| !part.is_empty())
                .collect();
            if vec.is_empty() {
                Ok(None)
            } else {
                Ok(Some(vec))
            }
        }
        Value::Array(arr) => {
            debug!("Input array: {:#?}", arr);
            let vec: Vec<String> = arr
                .into_iter()
                .filter_map(|v| v.as_str().map(|s| s.trim().to_string()))
                .filter(|s| !s.is_empty())
                .collect();
            if vec.is_empty() {
                Ok(None)
            } else {
                Ok(Some(vec))
            }
        }
        // 其他类型返回 None
        _ => {
            debug!("Unsupported type encountered");
            Ok(None)
        }
    }
}

// ------------------------------------------------------------------------
// Option 数字类型处理方法
// ------------------------------------------------------------------------

/// 将字符串反序列化为Option<T>，其中T为数字类型
///
/// # Arguments
///
/// * `deserializer`: D类型的反序列化器
///
/// # Returns
///
/// * `Result<Option<T>, D::Error>`: 转换后的数字类型，如果转换失败则返回None
pub fn string_to_option_number<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr + Bounded + NumCast,
    <T as FromStr>::Err: std::fmt::Display,
{
    let value = Value::deserialize(deserializer)?;

    match value {
        // 处理字符串类型
        Value::String(s) => match T::from_str(&s) {
            Ok(num) => Ok(Some(num)),
            Err(e) => {
                warn!("Failed to parse string to number: {}", e);
                Ok(None)
            }
        },
        // 处理数字类型
        Value::Number(num) => {
            if let Some(n) = num.as_i64() {
                if n >= T::min_value().to_i64().unwrap_or(i64::MIN)
                    && n <= T::max_value().to_i64().unwrap_or(i64::MAX)
                {
                    match NumCast::from(n) {
                        Some(num) => Ok(Some(num)),
                        None => {
                            warn!("Failed to cast number to target type");
                            Ok(None)
                        }
                    }
                } else {
                    warn!("Number out of range for target type");
                    Ok(None)
                }
            } else {
                warn!("Invalid number format");
                Ok(None)
            }
        }
        // 其他类型返回 None
        _ => Ok(None),
    }
}

// 泛型实现，减少重复代码
macro_rules! impl_string_to_option_number {
    ($func_name:ident, $type:ty) => {
        pub fn $func_name<'de, D>(deserializer: D) -> Result<Option<$type>, D::Error>
        where
            D: Deserializer<'de>,
        {
            string_to_option_number::<D, $type>(deserializer)
        }
    };
}

impl_string_to_option_number!(string_to_option_u8, u8);
impl_string_to_option_number!(string_to_option_i8, i8);
impl_string_to_option_number!(string_to_option_u16, u16);
impl_string_to_option_number!(string_to_option_i16, i16);
impl_string_to_option_number!(string_to_option_u32, u32);
impl_string_to_option_number!(string_to_option_i32, i32);
impl_string_to_option_number!(string_to_option_u64, u64);
impl_string_to_option_number!(string_to_option_i64, i64);

// ------------------------------------------------------------------------
// 数字类型处理方法
// ------------------------------------------------------------------------

/// 将字符串反序列化为T，其中T为数字类型，如果转换失败则返回默认值
///
/// # Arguments
///
/// * `deserializer`: D类型的反序列化器
///
/// # Returns
///
/// * `Result<T, D::Error>`: 转换后的数字类型，如果转换失败则返回默认值
pub fn string_to_number<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr + Bounded + NumCast + Zero,
    <T as FromStr>::Err: std::fmt::Display,
{
    let value = Value::deserialize(deserializer)?;
    let default_num = T::zero();

    match value {
        // 处理字符串类型
        Value::String(s) => match T::from_str(&s) {
            Ok(num) => Ok(num),
            Err(e) => {
                warn!("Failed to parse string to number: {}", e);
                Ok(default_num)
            }
        },
        // 处理数字类型
        Value::Number(num) => {
            if let Some(n) = num.as_i64() {
                if n >= T::min_value().to_i64().unwrap_or(i64::MIN)
                    && n <= T::max_value().to_i64().unwrap_or(i64::MAX)
                {
                    match NumCast::from(n) {
                        Some(num) => Ok(num),
                        None => {
                            warn!("Failed to cast number to target type");
                            Ok(default_num)
                        }
                    }
                } else {
                    warn!("Number out of range for target type");
                    Ok(default_num)
                }
            } else {
                warn!("Invalid number format");
                Ok(default_num)
            }
        }
        // 其他类型返回默认值
        _ => Ok(default_num),
    }
}

// 泛型实现，减少重复代码
macro_rules! impl_string_to_number {
    ($func_name:ident, $type:ty) => {
        pub fn $func_name<'de, D>(deserializer: D) -> Result<$type, D::Error>
        where
            D: Deserializer<'de>,
        {
            string_to_number::<D, $type>(deserializer)
        }
    };
}

impl_string_to_number!(string_to_u8, u8);
impl_string_to_number!(string_to_i8, i8);
impl_string_to_number!(string_to_u16, u16);
impl_string_to_number!(string_to_i16, i16);
impl_string_to_number!(string_to_u32, u32);
impl_string_to_number!(string_to_i32, i32);
impl_string_to_number!(string_to_u64, u64);
impl_string_to_number!(string_to_i64, i64);

// ------------------------------------------------------------------------
// 布尔类型处理方法
// ------------------------------------------------------------------------
/// 将字符串或其他类型转换为Option<bool>
///
/// 此函数主要用于反序列化过程中，将可能表示布尔值的字符串、数字等类型
/// 转换为Option<bool>类型，以支持更灵活的数据解析。
///
/// 参数:
/// - deserializer: D - 一个反序列化器，用于从数据源获取数据。
///
/// 返回:
/// - Result<Option<bool>, D::Error> - 转换后的布尔值，或错误。
pub fn string_to_option_bool<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;

    match value {
        // 处理布尔类型
        Value::Bool(b) => Ok(Some(b)),
        // 处理字符串类型
        Value::String(s) => match s.trim().to_lowercase().as_str() {
            "true" | "1" => Ok(Some(true)),
            "false" | "0" => Ok(Some(false)),
            _ => Ok(None),
        },
        // 处理数字类型
        Value::Number(num) => {
            if let Some(n) = num.as_i64() {
                match n {
                    1 => Ok(Some(true)),
                    0 => Ok(Some(false)),
                    _ => Ok(None),
                }
            } else {
                Ok(None)
            }
        }
        // 其他类型返回 None
        _ => Ok(None),
    }
}

// ------------------------------------------------------------------------
// 特定参数处理方法
// ------------------------------------------------------------------------
/// 将字符串转换为页码参数
///
/// 此函数专用于处理页码参数的反序列化，确保输入的字符串能够
/// 被正确地转换为u64类型的页码。
///
/// 参数:
/// - deserializer: D - 一个反序列化器，用于从数据源获取数据。
///
/// 返回:
/// - Result<u64, D::Error> - 转换后的页码参数，或错误。
pub fn string_to_param_page_no<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let res = string_to_number(deserializer);
    let num = match res {
        Ok(num) => parameter_utils::page_no_set(Some(num)),
        _ => parameter_utils::page_size_set(None),
    };
    Ok(num)
}

/// 将字符串转换为页面大小参数
///
/// 此函数专用于处理页面大小参数的反序列化，确保输入的字符串能够
/// 被正确地转换为u64类型的页面大小。
///
/// 参数:
/// - deserializer: D - 一个反序列化器，用于从数据源获取数据。
///
/// 返回:
/// - Result<u64, D::Error> - 转换后的页面大小参数，或错误。
pub fn string_to_param_page_size<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let res = string_to_number(deserializer);
    let num = match res {
        Ok(num) => parameter_utils::page_size_set(Some(num)),
        _ => parameter_utils::page_size_set(None),
    };
    Ok(num)
}

// ------------------------------------------------------------------------
// 时间类型处理方法
// ------------------------------------------------------------------------
/// 将字符串转换为Option<NaiveDateTime>
///
/// 此函数主要用于反序列化过程中，将表示日期时间的字符串转换为
/// Option<NaiveDateTime>类型，支持多种常见的日期时间格式。
///
/// 参数:
/// - deserializer: D - 一个反序列化器，用于从数据源获取数据。
///
/// 返回:
/// - Result<Option<NaiveDateTime>, D::Error> - 转换后的日期时间，或错误。
pub fn string_to_option_naive_datetime<'de, D>(
    deserializer: D,
) -> Result<Option<NaiveDateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;

    match value {
        Value::String(s) => {
            let formats = ["%Y-%m-%d %H:%M:%S", "%Y-%m-%dT%H:%M:%S"];
            for fmt in &formats {
                if let Ok(datetime) = NaiveDateTime::parse_from_str(s.trim(), fmt) {
                    return Ok(Some(datetime));
                }
            }
            warn!("Failed to parse datetime with supported formats");
            Ok(None)
        }
        _ => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDateTime;
    use serde_json::json;

    // -----------------------------
    // string_to_option_trimmed 测试
    // -----------------------------
    #[test]
    fn test_string_to_option_trimmed() {
        assert_eq!(
            string_to_option_trimmed(json!(" hello ")).unwrap(),
            Some("hello".to_string())
        );

        assert_eq!(string_to_option_trimmed(json!("")).unwrap(), None);

        assert_eq!(string_to_option_trimmed(json!(123)).unwrap(), None);
    }

    // -----------------------------
    // string_to_option_string_vec 测试
    // -----------------------------
    #[test]
    fn test_string_to_option_string_vec() {
        assert_eq!(
            string_to_option_string_vec(json!("a, b, c")).unwrap(),
            Some(vec!["a".to_string(), "b".to_string(), "c".to_string()])
        );

        assert_eq!(
            string_to_option_string_vec(json!(["a", " b ", 123])).unwrap(),
            Some(vec!["a".to_string(), "b".to_string()])
        );

        assert_eq!(string_to_option_string_vec(json!(123)).unwrap(), None);
    }

    // -----------------------------
    // string_to_option_number 测试
    // -----------------------------
    #[test]
    fn test_string_to_option_number() {
        assert_eq!(
            string_to_option_number::<_, i32>(json!("123")).unwrap(),
            Some(123)
        );

        assert_eq!(
            string_to_option_number::<_, i32>(json!(123)).unwrap(),
            Some(123)
        );

        assert_eq!(
            string_to_option_number::<_, i32>(json!("abc")).unwrap(),
            None
        );
    }

    // -----------------------------
    // string_to_number 测试
    // -----------------------------
    #[test]
    fn test_string_to_number() {
        assert_eq!(string_to_number::<_, i32>(json!("456")).unwrap(), 456);

        assert_eq!(string_to_number::<_, i32>(json!(456)).unwrap(), 456);

        assert_eq!(string_to_number::<_, i32>(json!("abc")).unwrap(), 0); // 默认值
    }

    // -----------------------------
    // string_to_option_bool 测试
    // -----------------------------
    #[test]
    fn test_string_to_option_bool() {
        assert_eq!(string_to_option_bool(json!("true")).unwrap(), Some(true));

        assert_eq!(string_to_option_bool(json!("1")).unwrap(), Some(true));

        assert_eq!(string_to_option_bool(json!(0)).unwrap(), Some(false));

        assert_eq!(string_to_option_bool(json!("invalid")).unwrap(), None);
    }

    // -----------------------------
    // string_to_option_naive_datetime 测试
    // -----------------------------
    #[test]
    fn test_string_to_option_naive_datetime() {
        let dt = NaiveDateTime::parse_from_str("2023-09-15 12:34:56", "%Y-%m-%d %H:%M:%S").unwrap();
        assert_eq!(
            string_to_option_naive_datetime(json!("2023-09-15 12:34:56")).unwrap(),
            Some(dt)
        );

        let dt = NaiveDateTime::parse_from_str("2023-09-15T12:34:56", "%Y-%m-%dT%H:%M:%S").unwrap();
        assert_eq!(
            string_to_option_naive_datetime(json!("2023-09-15T12:34:56")).unwrap(),
            Some(dt)
        );

        assert_eq!(
            string_to_option_naive_datetime(json!("invalid")).unwrap(),
            None
        );
    }
}
