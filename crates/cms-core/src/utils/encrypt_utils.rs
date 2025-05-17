use md5;

/// 加密密码字符串
///
/// 此函数通过将盐值和密码字符串拼接后，使用MD5哈希算法进行加密，返回加密后的十六进制字符串
/// 主要用于安全地存储用户密码
///
/// # 参数
///
/// * `salt` - 一个字符串切片，代表随机生成的盐值，用于增加密码的复杂度，防止预计算攻击
/// * `password` - 一个字符串切片，代表需要加密的用户密码
///
/// # 返回值
///
/// 返回一个String类型，包含加密后的密码
pub fn encrypt_password(salt: &str, password: &str) -> String {
    // 拼接盐值和密码
    let input = format!("{}{}", salt, password);

    // 计算MD5哈希值
    let digest = md5::compute(input);

    // 将哈希值转换为十六进制字符串
    let result = digest
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect::<String>();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_password_normal_input() {
        let salt = "salty";
        let password = "password123";
        let expected = "273a6bc42b2fb5af3859e54fec8ce7b9";
        assert_eq!(encrypt_password(salt, password), expected);
    }

    #[test]
    fn test_encrypt_password_empty_salt() {
        let salt = "";
        let password = "empty_salt";
        let expected = "6af3d530e0211f02acd15f15bada14f2";
        assert_eq!(encrypt_password(salt, password), expected);
    }

    #[test]
    fn test_encrypt_password_empty_password() {
        let salt = "empty_pwd";
        let password = "";
        let expected = "2b179aeff5e52bd16b1ee125b51d128a";
        assert_eq!(encrypt_password(salt, password), expected);
    }

    #[test]
    fn test_encrypt_password_both_empty() {
        let salt = "";
        let password = "";
        let expected = "d41d8cd98f00b204e9800998ecf8427e";
        assert_eq!(encrypt_password(salt, password), expected);
    }
}
