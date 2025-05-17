use rand::Rng;
use rand::distr::Alphanumeric;

/// 生成一个由字母和数字组成的随机字符串
///
/// # 参数
/// * `length`: 指定生成字符串的长度
///
/// # 返回值
/// 返回一个包含指定长度的随机字母数字字符串
///
/// # 注意事项
/// 如果 `length` 为 0，则返回空字符串
pub fn alpha_string(length: usize) -> String {
    if length == 0 {
        return String::new(); // 明确处理长度为 0 的情况
    }

    let mut rng = rand::rng();
    (0..length)
        .map(|_| rng.sample(Alphanumeric) as char)
        .collect() // 转换为字符串
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alpha_string_with_zero_length() {
        let result = alpha_string(0);
        assert_eq!(result, "", "Length 0 should return an empty string");
    }

    #[test]
    fn test_alpha_string_with_positive_length() {
        let length = 5;
        let result = alpha_string(length);
        assert_eq!(
            result.len(),
            length,
            "Generated string should have the correct length"
        );

        // Check that all characters are alphanumeric
        for c in result.chars() {
            assert!(
                c.is_ascii_alphanumeric(),
                "Character '{}' is not alphanumeric",
                c
            );
        }
    }

    #[test]
    fn test_alpha_string_with_length_one() {
        let result = alpha_string(1);
        assert_eq!(
            result.len(),
            1,
            "Length 1 should return a single character string"
        );

        let c = result.chars().next().unwrap();
        assert!(c.is_ascii_alphanumeric(), "Character '{}' is invalid", c);
    }

    #[test]
    fn test_alpha_string_with_large_length() {
        let length = 1000;
        let result = alpha_string(length);
        assert_eq!(
            result.len(),
            length,
            "Generated string should have length {}",
            length
        );

        for c in result.chars() {
            assert!(c.is_ascii_alphanumeric(), "Character '{}' is invalid", c);
        }
    }
}
