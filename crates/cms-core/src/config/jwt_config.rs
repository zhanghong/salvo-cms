use serde::Deserialize;
use tracing::warn;

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
        envy::prefixed("CMS_JWT_").from_env::<JwtConfig>()
    }

    /// 获取签名密钥的字节数组
    pub fn secret_bytes(&self) -> Vec<u8> {
        let mut key = self.secret_key.clone().unwrap_or_default();
        if key.is_empty() {
            key = "cms-secret".to_string();
        }
        key.as_bytes().to_vec()
    }

    /// 获取 Access Token 过期时间（天）
    pub fn get_access_expire_days(&self) -> i64 {
        let days = self.access_expire_days.unwrap_or(0);
        if days <= 0 {
            warn!(
                "Access token expiration days is not set or less than or equal to 0. Using default value of 7 days."
            );
            7
        } else if days > 30 {
            warn!(
                "Access token expiration days is greater than 30. Using default value of 30 days."
            );
            30
        } else {
            days
        }
    }

    /// 获取 Refresh Token 过期时间（天）
    pub fn get_refresh_expire_days(&self) -> i64 {
        let days = self.refresh_expire_days.unwrap_or(0);
        if days < 30 {
            warn!(
                "Access token expiration days is not set or less than or equal to 0. Using default value of 7 days."
            );
            30
        } else if days > 365 {
            warn!(
                "Access token expiration days is greater than 365. Using default value of 365 days."
            );
            365
        } else {
            days
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;

    #[test]
    fn test_jwt_config_from_env_all_set() {
        let mut config = JwtConfig::from_env().unwrap();
        let secret_key = env::var("CMS_JWT_SECRET_KEY").unwrap();
        let access_expire_days = env::var("CMS_JWT_ACCESS_EXPIRE_DAYS").unwrap();
        let assess_expire_days: i64 = access_expire_days.parse().unwrap();
        let refresh_expire_days = env::var("CMS_JWT_REFRESH_EXPIRE_DAYS").unwrap();
        let refresh_expire_days: i64 = refresh_expire_days.parse().unwrap();
        assert_eq!(config.get_access_expire_days(), assess_expire_days);
        assert_eq!(config.get_refresh_expire_days(), refresh_expire_days);
        assert_eq!(config.secret_bytes(), secret_key.as_bytes().to_vec());

        config.secret_key = None;
        let default_key_bytes = "cms-secret".as_bytes().to_vec();
        assert_eq!(config.secret_bytes(), default_key_bytes);
        config.secret_key = Some("".to_string());
        assert_eq!(config.secret_bytes(), default_key_bytes);

        config.access_expire_days = None;
        assert_eq!(config.get_access_expire_days(), 7);
        config.access_expire_days = Some(1);
        assert_eq!(config.get_access_expire_days(), 1);
        config.access_expire_days = Some(7);
        assert_eq!(config.get_access_expire_days(), 7);
        config.access_expire_days = Some(30);
        assert_eq!(config.get_access_expire_days(), 30);
        config.access_expire_days = Some(31);
        assert_eq!(config.get_access_expire_days(), 30);

        config.refresh_expire_days = None;
        assert_eq!(config.get_refresh_expire_days(), 30);
        config.refresh_expire_days = Some(1);
        assert_eq!(config.get_refresh_expire_days(), 30);
        config.refresh_expire_days = Some(30);
        assert_eq!(config.get_refresh_expire_days(), 30);
        config.refresh_expire_days = Some(365);
        assert_eq!(config.get_refresh_expire_days(), 365);
        config.refresh_expire_days = Some(366);
        assert_eq!(config.get_refresh_expire_days(), 365);
    }
}
