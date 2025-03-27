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
