use dotenvy::dotenv;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use serde::Deserialize;
use std::time::Duration;

use crate::{
    domain::{HandleResult, handle_ok},
    error::AppError,
};

/// 数据库配置结构体
#[derive(Deserialize, Debug)]
pub struct DbConfig {
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

impl DbConfig {
    /// 从环境变量中加载并解析数据库配置
    pub fn from_env() -> Result<Self, String> {
        // 尝试加载.env文件，如果失败则返回错误信息
        match dotenv() {
            Ok(_) => (),
            Err(e) => return Err(format!("Failed to load .env file: {}", e)),
        }
        // 从环境变量中解析DbConfig结构体，如果失败则返回错误信息
        envy::prefixed("CMS_DB_")
            .from_env::<DbConfig>()
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
