use num_traits::{Bounded, NumCast};
use serde::{Deserialize, Deserializer};
use serde_json::Value;
use std::str::FromStr;

pub fn string_to_option_trimmed<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;

    match value {
        // 处理字符串类型，去掉前后空格
        Value::String(s) => {
            let mut trimmed = s.trim();
            if trimmed.is_empty() {
                trimmed = "";
            }
            Ok(Some(trimmed.to_string()))
        }
        // 其他类型返回 None
        _ => Ok(None),
    }
}

pub fn string_to_option_number<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr + Bounded + NumCast,
{
    let value = Value::deserialize(deserializer)?;

    match value {
        // 处理字符串类型
        Value::String(s) => match T::from_str(&s) {
            Ok(num) => Ok(Some(num)),
            Err(_) => Ok(None),
        },
        // 处理数字类型
        Value::Number(num) => {
            if let Some(n) = num.as_i64() {
                if n >= T::min_value().to_i64().unwrap_or(i64::MIN)
                    && n <= T::max_value().to_i64().unwrap_or(i64::MAX)
                {
                    match NumCast::from(n) {
                        Some(num) => Ok(Some(num)),
                        None => Ok(None),
                    }
                } else {
                    Ok(None)
                }
            } else {
                Ok(None)
            }
        }
        // 其他类型返回 None
        _ => Ok(None),
    }
}

pub fn string_to_option_u8<'de, D>(deserializer: D) -> Result<Option<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    string_to_option_number(deserializer)
}

pub fn string_to_option_i8<'de, D>(deserializer: D) -> Result<Option<i8>, D::Error>
where
    D: Deserializer<'de>,
{
    string_to_option_number(deserializer)
}

pub fn string_to_option_u16<'de, D>(deserializer: D) -> Result<Option<u16>, D::Error>
where
    D: Deserializer<'de>,
{
    string_to_option_number(deserializer)
}

pub fn string_to_option_i16<'de, D>(deserializer: D) -> Result<Option<i16>, D::Error>
where
    D: Deserializer<'de>,
{
    string_to_option_number(deserializer)
}

pub fn string_to_option_u32<'de, D>(deserializer: D) -> Result<Option<u32>, D::Error>
where
    D: Deserializer<'de>,
{
    string_to_option_number(deserializer)
}

pub fn string_to_option_i32<'de, D>(deserializer: D) -> Result<Option<i32>, D::Error>
where
    D: Deserializer<'de>,
{
    string_to_option_number(deserializer)
}

pub fn string_to_option_u64<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
where
    D: Deserializer<'de>,
{
    string_to_option_number(deserializer)
}

pub fn string_to_option_i64<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
where
    D: Deserializer<'de>,
{
    string_to_option_number(deserializer)
}

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
