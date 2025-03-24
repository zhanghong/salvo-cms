use dotenvy::dotenv;
use lapin::{Connection, ConnectionProperties};
use serde::Deserialize;

use crate::domain::{handle_ok, HandleResult};

#[derive(Deserialize, Debug)]
pub struct RabbitMQConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub username: String,
    pub password: String,
    pub vhost: Option<String>,
    pub max_size: Option<usize>,
    pub min_size: Option<usize>,
    pub connect_timeout: Option<u64>,
    pub idle_timeout: Option<u64>,
    pub max_lifetime: Option<u64>,
}

impl RabbitMQConfig {
    pub fn from_env() -> Result<Self, envy::Error> {
        dotenv().ok();
        envy::prefixed("CMS_RABBITMQ_").from_env::<RabbitMQConfig>()
    }

    pub fn url(&self) -> String {
        format!(
            "amqp://{}:{}@{}:{}/{}",
            self.username,
            self.password,
            self.host.as_ref().unwrap_or(&"localhost".to_string()),
            self.port.unwrap_or(5672),
            self.vhost.as_ref().unwrap_or(&"/".to_string())
        )
    }

    pub async fn build_connection(&self) -> HandleResult<Connection> {
      let url = self.url();
      let conn: Connection = Connection::connect(url.as_str(), ConnectionProperties::default()).await.unwrap();

      handle_ok(conn)
    }
}