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
