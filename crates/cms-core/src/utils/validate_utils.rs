use lazy_static::lazy_static;
use num_traits::{Bounded, NumCast, Zero};
use regex::Regex;
use std::collections::HashMap;
use std::fmt::Debug;
use uuid::Uuid;
use validator::ValidationError;

// 提取公共函数：计算字符串长度
fn calculate_string_length(str: &str) -> usize {
    if str.is_empty() {
        0
    } else {
        str.chars().count()
    }
}

// 验证字符串是否为空
pub fn string_present(str: &str) -> Result<(), ValidationError> {
    let len = calculate_string_length(str);
    if len == 0 {
        return Err(ValidationError::new("length_invalid"));
    }
    Ok(())
}

// 验证字符串长度是否小于最小值
pub fn string_min_length(str: &str, min: usize) -> Result<(), ValidationError> {
    let len = calculate_string_length(str);
    if len < min {
        return Err(ValidationError::new("length_invalid"));
    }
    Ok(())
}

// 验证字符串长度是否大于最大值
pub fn string_max_length(str: &str, max: usize) -> Result<(), ValidationError> {
    let len = calculate_string_length(str);
    if len > max {
        return Err(ValidationError::new("length_invalid"));
    }
    Ok(())
}

// 综合验证字符串长度是否在指定范围内
pub fn string_length(
    str: &str,
    required: bool,
    min: usize,
    max: usize,
) -> Result<(), ValidationError> {
    let len = calculate_string_length(str);

    if !required && len == 0 {
        return Ok(());
    }

    if min > max {
        return Err(ValidationError::new("invalid_range"));
    }

    if (min > 0 && len < min) || (max > 0 && len > max) {
        return Err(ValidationError::new("length_invalid"));
    }

    Ok(())
}

pub fn string_uuid(str: &str) -> Result<(), ValidationError> {
    let len = calculate_string_length(str);
    if len != 36 {
        return Err(ValidationError::new("length_invalid"));
    }
    let uuid = Uuid::parse_str(str);
    if uuid.is_err() {
        return Err(ValidationError::new("uuid_invalid"));
    }
    Ok(())
}

pub fn numeric_greater_than_zero<T>(opt: Option<T>) -> Result<(), ValidationError>
where
    T: Copy + PartialOrd + Debug + Bounded + NumCast + Zero,
{
    if let Some(num) = opt {
        if num <= T::zero() {
            return Err(ValidationError::new("range_invalid"));
        }
        Ok(())
    } else {
        Ok(())
    }
}

// 验证数值是否等于或大于最小值
pub fn numeric_equal_or_greater_than<T: Copy + PartialOrd + Debug>(
    opt: Option<T>,
    min: T,
) -> Result<(), ValidationError> {
    if let Some(num) = opt {
        if num < min {
            return Err(ValidationError::new("range_invalid"));
        }
        Ok(())
    } else {
        Err(ValidationError::new("range_required"))
    }
}

// 验证数值是否等于或小于最大值
pub fn numeric_equal_or_less_than<T: Copy + PartialOrd + Debug>(
    opt: Option<T>,
    max: T,
) -> Result<(), ValidationError> {
    if let Some(num) = opt {
        if num > max {
            return Err(ValidationError::new("range_invalid"));
        }
        Ok(())
    } else {
        Err(ValidationError::new("range_required"))
    }
}

// 综合验证数值是否在指定范围内
pub fn numeric_range<T: Copy + PartialOrd + Debug>(
    opt: Option<T>,
    required: bool,
    min: T,
    max: T,
) -> Result<(), ValidationError> {
    if required && opt.is_none() {
        return Err(ValidationError::new("range_required"));
    }

    if let Some(num) = opt {
        if num < min || num > max {
            return Err(ValidationError::new("range_invalid"));
        }
        Ok(())
    } else {
        Ok(())
    }
}

// 使用正则表达式验证字符串格式
pub fn regex_string(str: &str, regex: &Regex, required: bool) -> bool {
    let len = if str.is_empty() { 0 } else { str.len() };
    if !required && len == 0 {
        return true;
    }
    regex.is_match(str)
}

// 定义常用的正则表达式
lazy_static! {
    static ref PHONE_REGEX: Regex = Regex::new(r"^1[3-9]\d{9}$").unwrap();
    static ref URL_REGEX: Regex = Regex::new(r"^(https?|http)://[^\s/$.?#].[^\s]*$").unwrap();
    static ref EMAIL_REGEX: Regex =
        Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
}

// 验证电话号码格式
pub fn phone_number(str: &str, required: bool) -> Result<(), ValidationError> {
    if regex_string(str, &PHONE_REGEX, required) {
        Ok(())
    } else {
        Err(ValidationError::new("phone_invalid"))
    }
}

// 验证URL地址格式
pub fn url_address(str: &str, required: bool) -> Result<(), ValidationError> {
    if regex_string(str, &URL_REGEX, required) {
        Ok(())
    } else {
        Err(ValidationError::new("url_invalid"))
    }
}

// 验证电子邮件地址格式
pub fn email_address(str: &str, required: bool) -> Result<(), ValidationError> {
    if regex_string(str, &EMAIL_REGEX, required) {
        Ok(())
    } else {
        Err(ValidationError::new("email_invalid"))
    }
}

// 验证枚举值是否有效
pub fn is_allow_enum_value(flag: bool) -> Result<(), ValidationError> {
    if flag {
        Ok(())
    } else {
        Err(ValidationError::new("invalid_enum_value"))
    }
}

// 验证哈希映射的长度是否在指定范围内
pub fn hash_map_max_length<T, N>(
    opt: Option<&&HashMap<T, N>>,
    max: usize,
) -> Result<(), ValidationError> {
    if let Some(map) = opt {
        if map.len() <= max {
            Ok(())
        } else {
            Err(ValidationError::new("hash out size"))
        }
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_calculate_string_length() {
        assert_eq!(calculate_string_length(""), 0);
        assert_eq!(calculate_string_length("abc"), 3);
        assert_eq!(calculate_string_length("你好"), 2); // Unicode chars
    }

    #[test]
    fn test_string_present() {
        assert!(string_present("").is_err());
        assert!(string_present("abc").is_ok());
    }

    #[test]
    fn test_string_min_length() {
        assert!(string_min_length("a", 2).is_err());
        assert!(string_min_length("abc", 2).is_ok());
    }

    #[test]
    fn test_string_max_length() {
        assert!(string_max_length("abcd", 3).is_err());
        assert!(string_max_length("ab", 3).is_ok());
    }

    #[test]
    fn test_string_length() {
        assert!(string_length("", true, 1, 5).is_err());
        assert!(string_length("", false, 1, 5).is_ok());
        assert!(string_length("a", true, 2, 5).is_err());
        assert!(string_length("abcdef", true, 2, 5).is_err());
        assert!(string_length("abc", true, 1, 5).is_ok());
        assert!(string_length("abc", true, 5, 3).is_err()); // min > max
    }

    #[test]
    fn test_string_uuid() {
        assert!(string_uuid("abc").is_err());
        assert!(string_uuid("111111111111111111111111111111111111").is_err());
        assert!(string_uuid("f904857e-706f-44a7-b917-998c28ec9ca8").is_ok());
    }

    #[test]
    fn test_numeric_greater_than_zero() {
        assert!(numeric_greater_than_zero::<i32>(Some(-1)).is_err());
        assert!(numeric_greater_than_zero(Some(0)).is_err());
        assert!(numeric_greater_than_zero(Some(1)).is_ok());
        assert!(numeric_greater_than_zero::<i32>(None).is_ok());
    }

    #[test]
    fn test_numeric_equal_or_greater_than() {
        assert!(numeric_equal_or_greater_than(Some(1), 2).is_err());
        assert!(numeric_equal_or_greater_than(Some(3), 2).is_ok());
        assert!(numeric_equal_or_greater_than::<i32>(None, 2).is_err());
    }

    #[test]
    fn test_numeric_equal_or_less_than() {
        assert!(numeric_equal_or_less_than(Some(5), 3).is_err());
        assert!(numeric_equal_or_less_than(Some(2), 3).is_ok());
        assert!(numeric_equal_or_less_than::<i32>(None, 3).is_err());
    }

    #[test]
    fn test_numeric_range() {
        assert!(numeric_range::<i32>(None, true, 1, 5).is_err());
        assert!(numeric_range(Some(0), true, 1, 5).is_err());
        assert!(numeric_range(Some(3), true, 1, 5).is_ok());
        assert!(numeric_range(Some(6), true, 1, 5).is_err());
        assert!(numeric_range(None, false, 1, 5).is_ok());
    }

    #[test]
    fn test_regex_string() {
        let phone_re = Regex::new(r"^1[3-9]\d{9}$").unwrap();
        assert!(regex_string("", &phone_re, false));
        assert!(regex_string("13800000000", &phone_re, true));
        assert!(!regex_string("12345", &phone_re, true));
    }

    #[test]
    fn test_phone_number() {
        assert!(phone_number("13800000000", true).is_ok());
        assert!(phone_number("12345678901", true).is_err());
        assert!(phone_number("", false).is_ok());
    }

    #[test]
    fn test_url_address() {
        assert!(url_address("http://example.com", true).is_ok());
        assert!(url_address("https://example.com/path?query=1", true).is_ok());
        assert!(url_address("example.com", true).is_err());
    }

    #[test]
    fn test_email_address() {
        assert!(email_address("user@example.com", true).is_ok());
        assert!(email_address("user.name+tag@sub.domain.com", true).is_ok());
        assert!(email_address("invalid-email@", true).is_err());
    }

    #[test]
    fn test_is_allow_enum_value() {
        assert!(is_allow_enum_value(true).is_ok());
        assert!(is_allow_enum_value(false).is_err());
    }

    #[test]
    fn test_hash_map_max_length() {
        let mut map = HashMap::new();
        map.insert(1, "a");
        map.insert(2, "b");

        assert!(hash_map_max_length(Some(&&map), 2).is_ok());
        assert!(hash_map_max_length(Some(&&map), 1).is_err());
        assert!(hash_map_max_length::<i32, &str>(None, 1).is_ok());
    }
}
