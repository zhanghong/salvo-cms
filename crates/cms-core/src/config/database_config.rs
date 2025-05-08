use dotenvy::dotenv;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use serde::Deserialize;
use std::time::Duration;
use tracing::warn;

use crate::{
    domain::{HandleResult, handle_ok},
    error::AppError,
};

/// 数据库配置结构体
#[derive(Deserialize, Debug)]
pub struct DatabaseConfig {
    pub protocol: Option<String>,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub user: String,
    pub password: String,
    pub name: String,
    pub schema: Option<String>,
    pub max_connections: Option<u32>,
    pub min_connections: Option<u32>,
    pub connect_timeout: Option<u64>,
    pub acquire_timeout: Option<u64>,
    pub idle_timeout: Option<u64>,
    pub max_lifetime: Option<u64>,
    pub sqlx_logging: Option<bool>,
}

impl DatabaseConfig {
    /// 从环境变量中加载并解析数据库配置
    pub fn from_env() -> Result<Self, String> {
        // 尝试加载 .env 文件，如果失败则记录警告日志
        if let Err(err) = dotenv() {
            warn!("Failed to load .env file: {}", err);
        }
        // 从环境变量中解析DatabaseConfig结构体，如果失败则返回错误信息
        envy::prefixed("CMS_DB_")
            .from_env::<DatabaseConfig>()
            .map_err(|e| format!("Failed to parse environment variables: {}", e))
    }

    /// 构建数据库连接URL
    pub fn url(&self) -> Result<String, String> {
        // 获取数据库协议，如果未指定则默认为mysql
        let protocol = self.protocol.as_deref().unwrap_or("mysql");
        // 检查数据库协议是否支持，如果不支持则返回错误信息
        if !["mysql", "postgres"].contains(&protocol) {
            return Err(format!("Unsupported database protocol: {}", protocol));
        }

        // 获取数据库主机，如果未指定则默认为localhost
        let host = self.host.as_deref().unwrap_or("localhost");
        // 检查数据库主机是否为空，如果为空则返回错误信息
        if host.is_empty() {
            return Err("Host cannot be empty".to_string());
        }

        // 根据数据库协议选择默认端口
        let port = self
            .port
            .unwrap_or_else(|| if protocol == "postgres" { 5432 } else { 3306 });

        // 获取数据库模式，如果未指定则根据协议选择默认值
        let schema =
            self.schema
                .as_deref()
                .unwrap_or(if protocol == "postgres" { "public" } else { "" });

        // 根据数据库协议构建连接URL
        if protocol == "postgres" {
            Ok(format!(
                "postgres://{}:{}@{}:{}/{}?schema={}",
                self.user, self.password, host, port, self.name, schema
            ))
        } else {
            Ok(format!(
                "mysql://{}:{}@{}:{}/{}",
                self.user, self.password, host, port, self.name
            ))
        }
    }

    /// 建立数据库连接
    pub async fn build_connection(&self) -> HandleResult<DatabaseConnection> {
        // 生成数据库连接URL，如果失败则返回错误信息
        let url = self
            .url()
            .map_err(|e| AppError::Database(format!("Failed to generate database URL: {}", e)))?;
        let mut opt = ConnectOptions::new(url);

        // 配置数据库连接选项
        opt.max_connections(self.max_connections.unwrap_or(10))
            .min_connections(self.min_connections.unwrap_or(5))
            .connect_timeout(Duration::from_secs(self.connect_timeout.unwrap_or(10)))
            .acquire_timeout(Duration::from_secs(self.acquire_timeout.unwrap_or(10)))
            .idle_timeout(Duration::from_secs(self.idle_timeout.unwrap_or(10)))
            .max_lifetime(Duration::from_secs(self.max_lifetime.unwrap_or(10)))
            .sqlx_logging(self.sqlx_logging.unwrap_or(true));

        // 建立数据库连接，如果失败则返回错误信息
        let connection = Database::connect(opt)
            .await
            .map_err(|e| AppError::Database(format!("Failed to connect to the database: {}", e)))?;

        handle_ok(connection)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    // 清除所有 CMS_DB_ 前缀的环境变量
    fn clear_env_vars() {
        unsafe {
            for key in env::vars()
                .filter(|(k, _)| k.starts_with("CMS_DB_"))
                .map(|(k, _)| k)
                .collect::<Vec<_>>()
            {
                env::remove_var(key);
            }
        }
    }

    #[test]
    fn test_from_env_success() {
        clear_env_vars();
        unsafe {
            env::set_var("CMS_DB_PROTOCOL", "postgres");
            env::set_var("CMS_DB_HOST", "db.example.com");
            env::set_var("CMS_DB_PORT", "5432");
            env::set_var("CMS_DB_USER", "admin");
            env::set_var("CMS_DB_PASSWORD", "secret");
            env::set_var("CMS_DB_NAME", "mydb");
            env::set_var("CMS_DB_SCHEMA", "public");
            env::set_var("CMS_DB_MAX_CONNECTIONS", "20");
            env::set_var("CMS_DB_MIN_CONNECTIONS", "5");
            env::set_var("CMS_DB_CONNECT_TIMEOUT", "30");
            env::set_var("CMS_DB_ACQUIRE_TIMEOUT", "20");
            env::set_var("CMS_DB_IDLE_TIMEOUT", "60");
            env::set_var("CMS_DB_MAX_LIFETIME", "3600");
            env::set_var("CMS_DB_SQLX_LOGGING", "false");
        }

        let config = DatabaseConfig::from_env().unwrap();

        assert_eq!(config.protocol, Some("postgres".to_string()));
        assert_eq!(config.host, Some("db.example.com".to_string()));
        assert_eq!(config.port, Some(5432));
        assert_eq!(config.user, "admin");
        assert_eq!(config.password, "secret");
        assert_eq!(config.name, "mydb");
        assert_eq!(config.schema, Some("public".to_string()));
        assert_eq!(config.max_connections, Some(20));
        assert_eq!(config.min_connections, Some(5));
        assert_eq!(config.connect_timeout, Some(30));
        assert_eq!(config.acquire_timeout, Some(20));
        assert_eq!(config.idle_timeout, Some(60));
        assert_eq!(config.max_lifetime, Some(3600));
        assert_eq!(config.sqlx_logging, Some(false));
    }

    #[test]
    fn test_url_default_mysql() {
        let config = DatabaseConfig {
            protocol: None,
            host: Some("localhost".to_string()),
            port: None,
            user: "root".to_string(),
            password: "password".to_string(),
            name: "test_db".to_string(),
            schema: None,
            max_connections: None,
            min_connections: None,
            connect_timeout: None,
            acquire_timeout: None,
            idle_timeout: None,
            max_lifetime: None,
            sqlx_logging: None,
        };

        let url = config.url().unwrap();
        assert_eq!(url, "mysql://root:password@localhost:3306/test_db");
    }

    #[test]
    fn test_url_postgres_with_schema() {
        let config = DatabaseConfig {
            protocol: Some("postgres".to_string()),
            host: Some("localhost".to_string()),
            port: Some(5432),
            user: "admin".to_string(),
            password: "pass".to_string(),
            name: "mydb".to_string(),
            schema: Some("custom".to_string()),
            max_connections: None,
            min_connections: None,
            connect_timeout: None,
            acquire_timeout: None,
            idle_timeout: None,
            max_lifetime: None,
            sqlx_logging: None,
        };

        let url = config.url().unwrap();
        assert_eq!(
            url,
            "postgres://admin:pass@localhost:5432/mydb?schema=custom"
        );
    }

    #[test]
    fn test_url_invalid_portoctl() {
        let config = DatabaseConfig {
            protocol: Some("sqlite".to_string()),
            host: Some("localhost".to_string()),
            port: None,
            user: "user".to_string(),
            password: "pass".to_string(),
            name: "db".to_string(),
            schema: None,
            max_connections: None,
            min_connections: None,
            connect_timeout: None,
            acquire_timeout: None,
            idle_timeout: None,
            max_lifetime: None,
            sqlx_logging: None,
        };

        let result = config.url();
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("Unsupported database protocol")
        );
    }

    #[test]
    fn test_url_empty_host() {
        let config = DatabaseConfig {
            protocol: None,
            host: Some("".to_string()),
            port: None,
            user: "user".to_string(),
            password: "pass".to_string(),
            name: "db".to_string(),
            schema: None,
            max_connections: None,
            min_connections: None,
            connect_timeout: None,
            acquire_timeout: None,
            idle_timeout: None,
            max_lifetime: None,
            sqlx_logging: None,
        };

        let result = config.url();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Host cannot be empty");
    }
}
