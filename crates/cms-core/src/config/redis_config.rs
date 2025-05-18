use dotenvy::dotenv;
use redis::{Client, aio::ConnectionManager};
use serde::Deserialize;
use tracing::{error, warn};

use crate::domain::{HandleResult, handle_ok};

#[derive(Deserialize, Debug)]
pub struct RedisConfig {
    host: Option<String>,
    port: Option<u16>,
    username: Option<String>,
    password: Option<String>,
    db: Option<u8>,
}

impl RedisConfig {
    /// 从环境变量中加载 Redis 配置
    pub fn from_env() -> Result<Self, envy::Error> {
        // 尝试加载 .env 文件，如果失败则记录警告日志
        if let Err(err) = dotenv() {
            warn!("Failed to load .env file: {}", err);
        }
        match envy::prefixed("CMS_REDIS_").from_env::<RedisConfig>() {
            Ok(config) => Ok(config),
            Err(err) => {
                error!("Failed to parse Redis configuration: {}", err);
                Err(err)
            }
        }
    }

    /// 构建 Redis 连接 URL
    pub fn url(&self) -> String {
        let username = self.username.as_deref().unwrap_or("");
        let password = self.password.as_deref().unwrap_or("");

        let prefix = if password.is_empty() {
            "".to_string()
        } else {
            format!("{}:{}@", username, password)
        };

        format!(
            "redis://{}{}:{}?db={}",
            prefix,
            self.host.as_deref().unwrap_or("localhost"),
            self.port.unwrap_or(6379),
            self.db.unwrap_or(0)
        )
    }

    pub async fn build_client(&self) -> HandleResult<Client> {
        let url = self.url();
        let client = Client::open(url).unwrap();

        handle_ok(client)
    }

    /// 构建 Redis 连接池
    pub async fn build_pool(&self) -> HandleResult<ConnectionManager> {
        let client = self.build_client().await?;

        let manager = ConnectionManager::new(client).await.unwrap();

        handle_ok(manager)
    }
}

// ... existing code ...

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    fn setup_env(vars: &[(&str, &str)]) {
        unsafe {
            for (key, value) in vars {
                env::set_var(key, value);
            }
        }
    }

    fn clear_env_vars() {
        let keys = [
            "CMS_REDIS_HOST",
            "CMS_REDIS_PORT",
            "CMS_REDIS_USERNAME",
            "CMS_REDIS_PASSWORD",
            "CMS_REDIS_DB",
        ];
        unsafe {
            for key in keys {
                env::remove_var(key);
            }
        }
    }

    #[test]
    fn test_redis_config_from_env() {
        clear_env_vars();
        setup_env(&[
            ("CMS_REDIS_HOST", "127.0.0.1"),
            ("CMS_REDIS_PORT", "6379"),
            ("CMS_REDIS_USERNAME", "user"),
            ("CMS_REDIS_PASSWORD", "pass"),
            ("CMS_REDIS_DB", "1"),
        ]);

        let config = RedisConfig::from_env().expect("Failed to load config");

        assert_eq!(config.host.as_deref().unwrap(), "127.0.0.1");
        assert_eq!(config.port.unwrap(), 6379);
        assert_eq!(config.username.as_deref().unwrap(), "user");
        assert_eq!(config.password.as_deref().unwrap(), "pass");
        assert_eq!(config.db.unwrap(), 1);
    }

    #[test]
    fn test_redis_config_url_with_auth() {
        let config = RedisConfig {
            host: Some("127.0.0.1".to_string()),
            port: Some(6379),
            username: Some("user".to_string()),
            password: Some("pass".to_string()),
            db: Some(1),
        };

        assert_eq!(config.url(), "redis://user:pass@127.0.0.1:6379?db=1");
    }

    #[test]
    fn test_redis_config_url_without_auth() {
        let config = RedisConfig {
            host: Some("127.0.0.1".to_string()),
            port: Some(6379),
            username: None,
            password: None,
            db: Some(1),
        };

        assert_eq!(config.url(), "redis://127.0.0.1:6379?db=1");
    }

    #[test]
    fn test_redis_config_default_values() {
        let config = RedisConfig {
            host: None,
            port: None,
            username: None,
            password: None,
            db: None,
        };

        assert_eq!(config.url(), "redis://localhost:6379?db=0");
    }

    #[tokio::test]
    async fn test_build_redis_client() {
        let config = RedisConfig {
            host: Some("127.0.0.1".to_string()),
            port: Some(6379),
            username: None,
            password: None,
            db: Some(0),
        };

        let result = config.build_client().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_build_redis_pool() {
        let config = RedisConfig {
            host: Some("127.0.0.1".to_string()),
            port: Some(6379),
            username: None,
            password: None,
            db: Some(0),
        };

        let result = config.build_pool().await;
        assert!(result.is_ok());
    }
}
