use deadpool_lapin::{Config, Pool, Runtime};
use tracing::{error, info};

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
        let username = self.username.as_deref().unwrap_or_default();
        let password = self.password.as_deref().unwrap_or_default();
        let mut host = self.host.as_deref().unwrap_or_default();
        if host.is_empty() {
            host = "localhost";
        }
        let mut port = self.port.unwrap_or_default();
        if port < 1 {
            port = 5672;
        }
        let mut vhost = self.vhost.as_deref().unwrap_or_default();
        if vhost.is_empty() {
            vhost = "/cms";
        }

        let prefix = if password.is_empty() {
            "".to_string()
        } else {
            format!("{}:{}@", username, password)
        };

        format!("amqp://{}{}:{}/{}", prefix, host, port, vhost)
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

    #[test]
    fn test_rabbitmq_config_from_env() {
        let config = RabbitMQConfig::from_env().expect("Failed to load config");

        let host = env::var("CMS_RABBITMQ_HOST");
        if host.is_ok() {
            assert_eq!(config.host.as_deref().unwrap(), host.unwrap());
        } else {
            assert!(config.host.is_none());
        }

        let port = env::var("CMS_RABBITMQ_PORT");
        if port.is_ok() {
            assert_eq!(config.port.unwrap(), port.unwrap().parse::<u16>().unwrap());
        } else {
            assert!(config.port.is_none());
        }

        let username = env::var("CMS_RABBITMQ_USERNAME");
        if username.is_ok() {
            assert_eq!(config.username.as_deref().unwrap(), username.unwrap());
        } else {
            assert!(config.username.is_none());
        }

        let password = env::var("CMS_RABBITMQ_PASSWORD");
        if password.is_ok() {
            assert_eq!(config.password.as_deref().unwrap(), password.unwrap());
        } else {
            assert!(config.password.is_none());
        }

        let vhost = env::var("CMS_RABBITMQ_VHOST");
        if vhost.is_ok() {
            assert_eq!(config.vhost.as_deref().unwrap(), vhost.unwrap());
        } else {
            assert!(config.vhost.is_none());
        }
    }

    #[tokio::test]
    async fn test_rabbitmq_config_url() {
        let mut config = RabbitMQConfig::from_env().expect("Failed to load config");
        let host = "127.0.0.1";
        let port = 5676u16;
        let username = "admin";
        let password = "123456";
        let vhost = "/cms-test";

        config.host = Some(host.to_string());
        config.port = Some(port);
        config.username = Some(username.to_string());
        config.password = Some(password.to_string());
        config.vhost = Some(vhost.to_string());

        let fmt_url = format!(
            "amqp://{}:{}@{}:{}/{}",
            username, password, host, port, vhost
        );
        assert_eq!(config.url(), fmt_url);

        config.host = None;
        let fmt_url = format!(
            "amqp://{}:{}@localhost:{}/{}",
            username, password, port, vhost
        );
        assert_eq!(config.url(), fmt_url);
        config.host = Some(host.to_string());
        config.host = Some("".to_string());
        let fmt_url = format!(
            "amqp://{}:{}@localhost:{}/{}",
            username, password, port, vhost
        );
        assert_eq!(config.url(), fmt_url);
        config.host = Some(host.to_string());
        config.host = Some(host.to_string());

        config.port = None;
        let fmt_url = format!("amqp://{}:{}@{}:5672/{}", username, password, host, vhost);
        assert_eq!(config.url(), fmt_url);
        config.port = Some(0u16);
        let fmt_url = format!("amqp://{}:{}@{}:5672/{}", username, password, host, vhost);
        assert_eq!(config.url(), fmt_url);
        config.port = Some(port);

        config.vhost = None;
        let fmt_url = format!("amqp://{}:{}@{}:{}//cms", username, password, host, port);
        assert_eq!(config.url(), fmt_url);
        config.vhost = Some("".to_string());
        let fmt_url = format!("amqp://{}:{}@{}:{}//cms", username, password, host, port);
        assert_eq!(config.url(), fmt_url);
        config.vhost = Some(vhost.to_string());

        config.password = None;
        let fmt_url = format!("amqp://{}:{}/{}", host, port, vhost);
        assert_eq!(config.url(), fmt_url);
        config.password = Some("".to_string());
        let fmt_url = format!("amqp://{}:{}/{}", host, port, vhost);
        assert_eq!(config.url(), fmt_url);
        config.password = Some(password.to_string());
    }

    #[tokio::test]
    async fn test_rabbitmq_config_build_pool() {
        let config = RabbitMQConfig::from_env().expect("Failed to load config");

        let result = config.build_pool().await;
        assert!(result.is_ok());
    }
}
