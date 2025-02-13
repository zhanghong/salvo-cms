use md5;
use std::fmt::Write;

pub fn encrypt_password(salt: &str, password: &str) -> String {
    let str = format!("{}{}", salt, password);
    let digest = md5::compute(str).to_vec();
    let mut result = String::new();
    for a in digest.iter() {
        write!(result, "{:02x}", a).unwrap();
    }
    result
}
