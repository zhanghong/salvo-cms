use deadpool_lapin::{Config, Pool, Runtime};
use dotenvy::dotenv;
use tracing::{error, info, warn};

use crate::{
    domain::{HandleResult, handle_ok},
    error::AppError,
};

#[derive(Debug, serde::Deserialize)]
pub struct RabbitMQPool {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub vhost: Option<String>,
}

impl RabbitMQPool {
    /// 从环境变量中加载 RabbitMQ 配置
    pub fn from_env() -> Result<Self, envy::Error> {
        // 尝试加载 .env 文件，如果失败则记录警告日志
        if let Err(err) = dotenv() {
            warn!("Failed to load .env file: {}", err);
        }
        match envy::prefixed("CMS_REDIS_").from_env::<RabbitMQPool>() {
            Ok(config) => Ok(config),
            Err(err) => {
                error!("Failed to parse Redis configuration: {}", err);
                Err(err)
            }
        }
    }

    /// 构建数据库连接URL
    pub fn url(&self) -> String {
        let username = self.username.as_deref().unwrap_or("");
        let password = self.password.as_deref().unwrap_or("");

        let prefix = if password.is_empty() {
            "".to_string()
        } else {
            format!("{}:{}@", username, password)
        };

        format!(
            "amqp://{}{}:{}/{}",
            prefix,
            self.host.as_deref().unwrap_or("localhost"),
            self.port.unwrap_or(5672),
            self.vhost.as_deref().unwrap_or("/cms")
        )
    }
    /// 构建 RabbitMQ 连接池
    pub async fn build_pool(&self) -> HandleResult<Pool> {
        let mut cfg = Config::default();

        let url = self.url();
        cfg.url = Some(url.to_owned());

        info!("Attempting to create RabbitMQ pool with URL: {}", url);

        // 创建连接池并处理可能的错误
        match cfg.create_pool(Some(Runtime::Tokio1)) {
            Ok(pool) => {
                info!("RabbitMQ pool created successfully.");
                handle_ok(pool)
            }
            Err(err) => {
                error!("Failed to create RabbitMQ pool: {}", err);
                Err(AppError::Queue(format!(
                    "Failed to create RabbitMQ pool: {}",
                    err
                )))
            }
        }
    }
}
