use std::fmt::Debug;

use regex::Regex;
use validator::ValidationError;

pub fn string_length(
    str: &str,
    required: bool,
    min: usize,
    max: usize,
) -> Result<(), ValidationError> {
    let len = if str.is_empty() { 0 } else { str.len() };

    if !required && len == 0 {
        return Ok(());
    }
    if (min > 0 && len < min) || (max > 0 && len > max) {
        return Err(ValidationError::new("length_invalid"));
    }

    Ok(())
}

pub fn numeric_range<T: Copy + PartialOrd + Debug>(
    opt: Option<T>,
    required: bool,
    min: T,
    max: T,
) -> Result<(), ValidationError> {
    if required && opt.is_none() {
        return Err(ValidationError::new("range_required"));
    } else if opt.is_none() {
        return Ok(());
    }

    let num = opt.unwrap();
    if num < min || num > max {
        return Err(ValidationError::new("range_invalid"));
    }

    Ok(())
}

pub fn regex_string(str: &str, regex: &Regex, required: bool) -> bool {
    let len = if str.is_empty() { 0 } else { str.len() };
    if !required && len == 0 {
        return true;
    }

    regex.is_match(str)
}

pub fn phone_number(str: &str, required: bool) -> Result<(), ValidationError> {
    let regex = Regex::new(r"^1[3-9]\d{9}$").unwrap();
    if regex_string(str, &regex, required) {
        Ok(())
    } else {
        Err(ValidationError::new("phone_invalid"))
    }
}

pub fn url_address(str: &str, required: bool) -> Result<(), ValidationError> {
    let regex = Regex::new(r"^(https?|http)://[^\s/$.?#].[^\s]*$").unwrap();
    if regex_string(str, &regex, required) {
        Ok(())
    } else {
        Err(ValidationError::new("phone_invalid"))
    }
}

pub fn email_address(str: &str, required: bool) -> Result<(), ValidationError> {
    let regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    if regex_string(str, &regex, required) {
        Ok(())
    } else {
        Err(ValidationError::new("phone_invalid"))
    }
}

pub fn is_allow_enum_value(flag: bool) -> Result<(), ValidationError> {
    if flag {
        Ok(())
    } else {
        Err(ValidationError::new("invalid_enum_value"))
    }
}
