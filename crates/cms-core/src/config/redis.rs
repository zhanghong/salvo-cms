use dotenvy::dotenv;
use redis::{aio::ConnectionManager, Client};
use serde::Deserialize;

use crate::domain::{handle_ok, HandleResult};

#[derive(Deserialize, Debug)]
pub struct RedisConfig {
    host: Option<String>,
    port: Option<u16>,
    username: Option<String>,
    password: Option<String>,
    db: Option<u8>,
    // pool_max_size: Option<u32>,
    // connection_timeout: Option<u64>,
}

impl RedisConfig {
    pub fn from_env() -> Result<Self, envy::Error> {
        dotenv().ok();
        envy::prefixed("REDIS_").from_env::<RedisConfig>()
    }

    pub fn url(&self) -> String {
        let username = match self.username.clone() {
            Some(username) => username,
            None => String::from(""),
        };
        let password = match self.password.clone() {
            Some(password) => password,
            None => String::from(""),
        };
        let prefix = if password.is_empty() {
            "".to_string()
        } else {
            format!("{}:{}@", username, password)
        };

        format!(
            "redis://{}{}:{}?db={}",
            prefix,
            self.host.as_ref().unwrap_or(&"localhost".to_string()),
            self.port.unwrap_or(6379),
            self.db.unwrap_or(0)
        )
    }

    pub async fn build_client(&self) -> HandleResult<Client> {
        let url = self.url();
        let client = Client::open(url).unwrap();

        handle_ok(client)
    }

    pub async fn build_pool(&self) -> HandleResult<ConnectionManager> {
        let url = self.url();
        let client = Client::open(url.to_owned()).unwrap();

        let manager = ConnectionManager::new(client).await.unwrap();

        handle_ok(manager)
    }
}
