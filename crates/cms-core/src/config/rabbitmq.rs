use deadpool_lapin::{Config, Pool, Runtime};
use dotenvy::dotenv;

use crate::domain::{HandleResult, handle_ok};

#[derive(Debug, serde::Deserialize)]
pub struct RabbitMQPool {}

impl RabbitMQPool {
    pub fn from_env() -> Result<Self, envy::Error> {
        dotenv().ok();
        envy::prefixed("CMS_RABBITMQ_").from_env::<RabbitMQPool>()
    }
    pub async fn build_pool(&self) -> HandleResult<Pool> {
        let mut cfg = Config::default();
        cfg.url = Some("amqp://guest:guest@rabbitmq:5672//cms".into());
        let pool = cfg.create_pool(Some(Runtime::Tokio1))?;
        handle_ok(pool)
    }
}
