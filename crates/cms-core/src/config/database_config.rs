use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use serde::Deserialize;
use std::time::Duration;

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
    use lapin::protocol;
    use salvo::oapi::security::Password;

    use super::*;
    use std::env;

    #[tokio::test]
    async fn test_database_config_from_env() {
        let config = DatabaseConfig::from_env().unwrap();

        let protocol = env::var("CMS_DB_PROTOCOL");
        if protocol.is_ok() {
            assert_eq!(config.protocol.unwrap(), protocol.unwrap());
        } else {
            assert!(config.protocol.is_none());
        }

        let host = env::var("CMS_DB_HOST");
        if host.is_ok() {
            assert_eq!(config.host.unwrap(), host.unwrap());
        } else {
            assert!(config.host.is_none());
        }

        let port = env::var("CMS_DB_PORT");
        if port.is_ok() {
            assert_eq!(config.port.unwrap(), port.unwrap().parse::<u16>().unwrap());
        } else {
            assert!(config.port.is_none());
        }

        let user = env::var("CMS_DB_USER");
        assert_eq!(config.user, user.unwrap());

        let password = env::var("CMS_DB_PASSWORD");
        assert_eq!(config.password, password.unwrap());

        let name = env::var("CMS_DB_NAME");
        assert_eq!(config.name, name.unwrap());

        let schema = env::var("CMS_DB_SCHEMA");
        if schema.is_ok() {
            assert_eq!(config.schema.unwrap(), schema.unwrap());
        } else {
            assert!(config.schema.is_none());
        }

        let max_connections = env::var("CMS_DB_MAX_CONNECTIONS");
        if max_connections.is_ok() {
            assert_eq!(
                config.max_connections.unwrap(),
                max_connections.unwrap().parse::<u32>().unwrap()
            );
        } else {
            assert!(config.max_connections.is_none());
        }

        let min_connections = env::var("CMS_DB_MIN_CONNECTIONS");
        if min_connections.is_ok() {
            assert_eq!(
                config.min_connections.unwrap(),
                min_connections.unwrap().parse::<u32>().unwrap()
            );
        } else {
            assert!(config.min_connections.is_none());
        }

        let connect_timeout = env::var("CMS_DB_CONNECT_TIMEOUT");
        if connect_timeout.is_ok() {
            assert_eq!(
                config.connect_timeout.unwrap(),
                connect_timeout.unwrap().parse::<u64>().unwrap()
            );
        } else {
            assert!(config.connect_timeout.is_none());
        }

        let acquire_timeout = env::var("CMS_DB_ACQUIRE_TIMEOUT");
        if acquire_timeout.is_ok() {
            assert_eq!(
                config.acquire_timeout.unwrap(),
                acquire_timeout.unwrap().parse::<u64>().unwrap()
            );
        } else {
            assert!(config.acquire_timeout.is_none());
        }

        let idle_timeout = env::var("CMS_DB_IDLE_TIMEOUT");
        if idle_timeout.is_ok() {
            assert_eq!(
                config.idle_timeout.unwrap(),
                idle_timeout.unwrap().parse::<u64>().unwrap()
            );
        } else {
            assert!(config.idle_timeout.is_none());
        }

        let max_lifetime = env::var("CMS_DB_MAX_LIFETIME");
        if max_lifetime.is_ok() {
            assert_eq!(
                config.max_lifetime.unwrap(),
                max_lifetime.unwrap().parse::<u64>().unwrap()
            );
        } else {
            assert!(config.max_lifetime.is_none());
        }

        let sqlx_logging = env::var("CMS_DB_SQLX_LOGGING");
        if sqlx_logging.is_ok() {
            assert_eq!(
                config.sqlx_logging.unwrap(),
                sqlx_logging.unwrap().parse::<bool>().unwrap()
            );
        } else {
            assert!(config.sqlx_logging.is_none());
        }
    }

    #[tokio::test]
    async fn test_database_protocol_painc() {
        let mut config = DatabaseConfig::from_env().unwrap();
        config.protocol = Some("none".to_string());
        let db_url = config.url();
        assert!(db_url.is_err());
        assert_eq!(
            db_url.unwrap_err(),
            "Unsupported database protocol: none".to_string()
        );
    }

    #[tokio::test]
    async fn test_data_host_painc() {
        let mut config = DatabaseConfig::from_env().unwrap();
        config.host = Some("".to_string());
        let db_url = config.url();
        assert!(db_url.is_err());
        assert_eq!(db_url.unwrap_err(), "Host cannot be empty".to_string());
    }

    #[tokio::test]
    async fn test_mysql_database_config_url() {
        let mut config = DatabaseConfig::from_env().unwrap();
        config.protocol = Some("mysql".to_string());
        let host = "localhost";
        let port = 3306u16;
        let user = "root";
        let password = "password";
        let name = "salvo_cms";

        config.host = Some(host.to_string());
        config.port = Some(port);
        config.user = user.to_string();
        config.password = password.to_string();
        config.name = name.to_string();
        let db_url = format!("mysql://{}:{}@{}:{}/{}", user, password, host, port, name);
        let cfg_url = config.url().unwrap();
        assert_eq!(cfg_url, db_url);

        config.host = None;
        let db_url = format!("mysql://{}:{}@{}:{}/{}", user, password, host, port, name);
        let cfg_url = config.url().unwrap();
        assert_eq!(cfg_url, db_url);
        config.host = Some(host.to_string());

        config.host = Some("127.0.0.1".to_string());
        let db_url = format!(
            "mysql://{}:{}@{}:{}/{}",
            user, password, "127.0.0.1", port, name
        );
        let cfg_url = config.url().unwrap();
        assert_eq!(cfg_url, db_url);
        config.host = Some(host.to_string());

        config.port = None;
        config.port = Some(13306);
        let db_url = format!("mysql://{}:{}@{}:{}/{}", user, password, host, 13306, name);
        let cfg_url = config.url().unwrap();
        assert_eq!(cfg_url, db_url);
        config.port = Some(port);
    }

    #[tokio::test]
    async fn test_postgres_database_config_url() {
        let mut config = DatabaseConfig::from_env().unwrap();
        config.protocol = Some("postgres".to_string());
        let host = "localhost";
        let port = 5432u16;
        let user = "root";
        let password = "password";
        let name = "salvo_cms";
        let schema = "public";

        config.host = Some(host.to_string());
        config.port = Some(port);
        config.user = user.to_string();
        config.password = password.to_string();
        config.name = name.to_string();
        config.schema = Some(schema.to_string());
        let db_url = format!(
            "postgres://{}:{}@{}:{}/{}?schema={}",
            user, password, host, port, name, schema
        );
        let cfg_url = config.url().unwrap();
        assert_eq!(cfg_url, db_url);

        config.host = None;
        let db_url = format!(
            "postgres://{}:{}@{}:{}/{}?schema={}",
            user, password, host, port, name, schema
        );
        let cfg_url = config.url().unwrap();
        assert_eq!(cfg_url, db_url);
        config.host = Some(host.to_string());

        config.host = Some("127.0.0.1".to_string());
        let db_url = format!(
            "postgres://{}:{}@{}:{}/{}?schema={}",
            user, password, "127.0.0.1", port, name, schema
        );
        let cfg_url = config.url().unwrap();
        assert_eq!(cfg_url, db_url);
        config.host = Some(host.to_string());

        config.port = None;
        config.port = Some(15432);
        let db_url = format!(
            "postgres://{}:{}@{}:{}/{}?schema={}",
            user, password, host, 15432, name, schema
        );
        let cfg_url = config.url().unwrap();
        assert_eq!(cfg_url, db_url);
        config.port = Some(port);
    }
}
