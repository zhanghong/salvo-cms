use lazy_static::lazy_static;
use num_traits::{Bounded, NumCast, Zero};
use regex::Regex;
use std::collections::HashMap;
use std::fmt::Debug;
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
