use dotenvy::dotenv;
use serde::Deserialize;
use tracing::{error, warn};

#[derive(Deserialize, Debug, Clone)]
pub struct JwtConfig {
    /// JWT 签名密钥，默认值为 "Cms Jwt Secret Key"，建议用户自定义
    secret_key: Option<String>,
    /// Access Token 过期时间（天），默认值为 7 天
    access_expire_days: Option<i64>,
    /// Refresh Token 过期时间（天），默认值为 365 天
    refresh_expire_days: Option<i64>,
}

impl JwtConfig {
    /// 从环境变量中加载配置
    pub fn from_env() -> Result<Self, envy::Error> {
        // 尝试加载 .env 文件，如果失败则记录警告日志
        if let Err(err) = dotenv() {
            warn!("Failed to load .env file: {}", err);
        }
        envy::prefixed("CMS_JWT_").from_env::<JwtConfig>()
    }

    /// 获取签名密钥的字节数组
    pub fn secret_bytes(&self) -> Vec<u8> {
        self.secret_key().as_bytes().to_vec()
    }

    /// 获取 Access Token 过期时间（天）
    pub fn get_access_expire_days(&self) -> i64 {
        self.access_expire_days()
    }

    /// 获取 Refresh Token 过期时间（天）
    pub fn get_refresh_expire_days(&self) -> i64 {
        self.refresh_expire_days()
    }

    /// 获取签名密钥，默认值为 "Cms Jwt Secret Key"
    fn secret_key(&self) -> String {
        self.secret_key.clone().unwrap_or_else(|| {
            error!("JWT secret key is not set in environment variables. Using default value.");
            "Cms Jwt Secret Key".to_string()
        })
    }

    /// 获取 Access Token 过期时间，默认值为 7 天
    fn access_expire_days(&self) -> i64 {
        self.access_expire_days
            .unwrap_or_else(|| {
                warn!("Access token expiration days not set. Using default value of 7 days.");
                7
            })
            .max(0) // 确保值非负
    }

    /// 获取 Refresh Token 过期时间，默认值为 365 天
    fn refresh_expire_days(&self) -> i64 {
        self.refresh_expire_days
            .unwrap_or_else(|| {
                warn!("Refresh token expiration days not set. Using default value of 365 days.");
                365
            })
            .max(0) // 确保值非负
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    fn clear_env_vars() {
        unsafe {
            for key in env::vars()
                .filter(|(k, _)| k.starts_with("CMS_JWT_"))
                .map(|(k, _)| k)
                .collect::<Vec<_>>()
            {
                env::remove_var(key);
            }
        }
    }

    #[test]
    fn test_jwt_config_from_env_all_set() {
        clear_env_vars();
        unsafe {
            env::set_var("CMS_JWT_SECRET_KEY", "my-secret");
            env::set_var("CMS_JWT_ACCESS_EXPIRE_DAYS", "10");
            env::set_var("CMS_JWT_REFRESH_EXPIRE_DAYS", "30");
        }

        let config = JwtConfig::from_env().unwrap();

        assert_eq!(config.secret_key(), "my-secret");
        assert_eq!(config.get_access_expire_days(), 10);
        assert_eq!(config.get_refresh_expire_days(), 30);
    }

    #[test]
    fn test_secret_bytes_with_custom_key() {
        let config = JwtConfig {
            secret_key: Some("my_secret".to_string()),
            access_expire_days: None,
            refresh_expire_days: None,
        };
        assert_eq!(config.secret_bytes(), b"my_secret");
    }

    #[test]
    fn test_secret_bytes_with_default_key() {
        let config = JwtConfig {
            secret_key: None,
            access_expire_days: None,
            refresh_expire_days: None,
        };
        assert_eq!(config.secret_bytes(), b"Cms Jwt Secret Key");
    }

    #[test]
    fn test_get_access_expire_days_with_custom_value() {
        let config = JwtConfig {
            secret_key: None,
            access_expire_days: Some(10),
            refresh_expire_days: None,
        };
        assert_eq!(config.get_access_expire_days(), 10);
    }

    #[test]
    fn test_get_access_expire_days_with_default_value() {
        let config = JwtConfig {
            secret_key: None,
            access_expire_days: None,
            refresh_expire_days: None,
        };
        assert_eq!(config.get_access_expire_days(), 7);
    }

    #[test]
    fn test_get_access_expire_days_with_negative_value() {
        let config = JwtConfig {
            secret_key: None,
            access_expire_days: Some(-1),
            refresh_expire_days: None,
        };
        assert_eq!(config.get_access_expire_days(), 0);
    }

    #[test]
    fn test_get_refresh_expire_days_with_custom_value() {
        let config = JwtConfig {
            secret_key: None,
            access_expire_days: None,
            refresh_expire_days: Some(100),
        };
        assert_eq!(config.get_refresh_expire_days(), 100);
    }

    #[test]
    fn test_get_refresh_expire_days_with_default_value() {
        let config = JwtConfig {
            secret_key: None,
            access_expire_days: None,
            refresh_expire_days: None,
        };
        assert_eq!(config.get_refresh_expire_days(), 365);
    }

    #[test]
    fn test_get_refresh_expire_days_with_negative_value() {
        let config = JwtConfig {
            secret_key: None,
            access_expire_days: None,
            refresh_expire_days: Some(-10),
        };
        assert_eq!(config.get_refresh_expire_days(), 0);
    }
}
