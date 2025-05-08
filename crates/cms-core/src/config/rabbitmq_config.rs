use deadpool_lapin::{Config, Pool, Runtime};
use dotenvy::dotenv;
use tracing::{error, info, warn};

use crate::{
    domain::{HandleResult, handle_ok},
    error::AppError,
};

#[derive(Debug, serde::Deserialize)]
pub struct RabbitMQConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub vhost: Option<String>,
}

impl RabbitMQConfig {
    /// 从环境变量中加载 RabbitMQ 配置
    pub fn from_env() -> Result<Self, envy::Error> {
        // 尝试加载 .env 文件，如果失败则记录警告日志
        if let Err(err) = dotenv() {
            warn!("Failed to load .env file: {}", err);
        }
        match envy::prefixed("CMS_RABBITMQ_").from_env::<RabbitMQConfig>() {
            Ok(config) => Ok(config),
            Err(err) => {
                error!("Failed to parse RabbitMQ configuration: {}", err);
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
            "CMS_RABBITMQ_HOST",
            "CMS_RABBITMQ_PORT",
            "CMS_RABBITMQ_USERNAME",
            "CMS_RABBITMQ_PASSWORD",
            "CMS_RABBITMQ_VHOST",
        ];
        unsafe {
            for key in keys {
                env::remove_var(key);
            }
        }
    }

    #[test]
    fn test_rabbitmq_config_from_env() {
        clear_env_vars();
        setup_env(&[
            ("CMS_RABBITMQ_HOST", "127.0.0.1"),
            ("CMS_RABBITMQ_PORT", "5672"),
            ("CMS_RABBITMQ_USERNAME", "guest"),
            ("CMS_RABBITMQ_PASSWORD", "guest"),
            ("CMS_RABBITMQ_VHOST", "/test"),
        ]);

        let config = RabbitMQConfig::from_env().expect("Failed to load config");

        assert_eq!(config.host.as_deref().unwrap(), "127.0.0.1");
        assert_eq!(config.port.unwrap(), 5672);
        assert_eq!(config.username.as_deref().unwrap(), "guest");
        assert_eq!(config.password.as_deref().unwrap(), "guest");
        assert_eq!(config.vhost.as_deref().unwrap(), "/test");
    }

    #[test]
    fn test_rabbitmq_config_url_with_auth() {
        let config = RabbitMQConfig {
            host: Some("127.0.0.1".to_string()),
            port: Some(5672),
            username: Some("guest".to_string()),
            password: Some("guest".to_string()),
            vhost: Some("/test".to_string()),
        };

        assert_eq!(config.url(), "amqp://guest:guest@127.0.0.1:5672//test");
    }

    #[test]
    fn test_rabbitmq_config_url_without_auth() {
        let config = RabbitMQConfig {
            host: Some("127.0.0.1".to_string()),
            port: Some(5672),
            username: None,
            password: None,
            vhost: Some("/test".to_string()),
        };

        assert_eq!(config.url(), "amqp://127.0.0.1:5672//test");
    }

    #[test]
    fn test_rabbitmq_config_default_values() {
        let config = RabbitMQConfig {
            host: None,
            port: None,
            username: None,
            password: None,
            vhost: None,
        };

        assert_eq!(config.url(), "amqp://localhost:5672//cms");
    }

    #[tokio::test]
    async fn test_build_pool() {
        let config = RabbitMQConfig {
            host: Some("127.0.0.1".to_string()),
            port: Some(5672),
            username: Some("guest".to_string()),
            password: Some("guest".to_string()),
            vhost: Some("/".to_string()),
        };

        let result = config.build_pool().await;
        assert!(result.is_ok());
    }
}
