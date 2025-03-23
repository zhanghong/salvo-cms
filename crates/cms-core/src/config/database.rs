use dotenvy::dotenv;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use serde::Deserialize;
use std::time::Duration;

use crate::domain::{handle_ok, HandleResult};

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
    pub fn from_env() -> Result<Self, envy::Error> {
        dotenv().ok();
        envy::prefixed("CMS_DB_").from_env::<DbConfig>()
    }

    pub fn url(&self) -> String {
        let prot = match self.protocol.as_ref() {
            Some(str) => str.as_str(),
            None => "mysql",
        };
        if "postgres".eq(prot) {
            format!(
                "postgres://{}:{}@{}:{}/{}?schema={}",
                self.user,
                self.password,
                self.host.as_ref().unwrap_or(&"localhost".to_string()),
                self.port.unwrap_or(5432),
                self.name,
                self.schema.as_ref().unwrap_or(&"public".to_string())
            )
        } else {
            format!(
                "mysql://{}:{}@{}:{}/{}",
                self.user,
                self.password,
                self.host.as_ref().unwrap_or(&"localhost".to_string()),
                self.port.unwrap_or(3306),
                self.name
            )
        }
    }

    pub async fn build_connection(&self) -> HandleResult<DatabaseConnection> {
        let mut opt = ConnectOptions::new(self.url());
        opt.max_connections(self.max_connections.unwrap_or(10))
            .min_connections(self.min_connections.unwrap_or(10))
            .connect_timeout(Duration::from_secs(
                self.connect_timeout.unwrap_or(10) as u64
            ))
            .acquire_timeout(Duration::from_secs(
                self.acquire_timeout.unwrap_or(10) as u64
            ))
            .idle_timeout(Duration::from_secs(self.idle_timeout.unwrap_or(10) as u64))
            .max_lifetime(Duration::from_secs(self.max_lifetime.unwrap_or(10) as u64))
            .sqlx_logging(self.sqlx_logging.clone().unwrap_or(true));
        let db = Database::connect(opt).await.unwrap();

        handle_ok(db)
    }
}
