use redis::{Client, aio::ConnectionManager};
use serde::Deserialize;
use tracing::error;

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
    use std::env;

    use super::*;

    #[tokio::test]
    async fn test_redis_config() {
        let mut config = RedisConfig::from_env().expect("Failed to load config");

        let host = env::var("CMS_REDIS_HOST");
        if host.is_ok() {
            assert_eq!(config.host.as_deref().unwrap(), host.unwrap());
        } else {
            assert_eq!(config.host, None);
        }
        let port = env::var("CMS_REDIS_PORT");
        if port.is_ok() {
            assert_eq!(config.port.unwrap(), port.unwrap().parse::<u16>().unwrap());
        } else {
            assert_eq!(config.port, None);
        }
        let username = env::var("CMS_REDIS_USERNAME");
        if username.is_ok() {
            assert_eq!(config.username.as_deref().unwrap(), username.unwrap());
        } else {
            assert_eq!(config.username, None);
        }
        let password = env::var("CMS_REDIS_PASSWORD");
        if password.is_ok() {
            assert_eq!(config.password.as_deref().unwrap(), password.unwrap());
        } else {
            assert_eq!(config.password, None);
        }
        let db = env::var("CMS_REDIS_DB");
        if db.is_ok() {
            assert_eq!(config.db.unwrap(), db.unwrap().parse::<u8>().unwrap());
        } else {
            assert_eq!(config.db, None);
        }
        let client = config.build_client().await;
        assert!(client.is_ok());

        config.host = Some("127.0.0.1".to_string());
        config.port = Some(6379);
        config.password = Some("secret".to_string());
        config.username = Some("user".to_string());
        config.db = Some(0);
        assert_eq!(config.url(), "redis://user:secret@127.0.0.1:6379?db=0");
        config.username = None;
        assert_eq!(config.url(), "redis://:secret@127.0.0.1:6379?db=0");
        config.username = Some("".to_string());
        config.password = Some("123456".to_string());
        assert_eq!(config.url(), "redis://:123456@127.0.0.1:6379?db=0");
        config.username = Some("root".to_string());
        config.password = Some("".to_string());
        assert_eq!(config.url(), "redis://127.0.0.1:6379?db=0");
    }
}
