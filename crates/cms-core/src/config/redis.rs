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
