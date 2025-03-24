use redis::Client;
use sea_orm::DatabaseConnection;

use super::DbConfig;
use super::RedisConfig;
use super::RabbitMQConfig;

#[derive(Clone, Debug)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub redis: Client,
    pub rabbitmq: String,
}

impl AppState {
    pub async fn init() -> Self {
        let db_config = DbConfig::from_env().expect("Failed to load db config");
        let redis_config = RedisConfig::from_env().expect("Failed to load redis config");
        let rabbitmq_config = RabbitMQConfig::from_env().expect("Failed to load rabbitmq config");

        let db = db_config.build_connection().await.unwrap();
        let redis = redis_config.build_client().await.unwrap();
        let rabbitmq = rabbitmq_config.url();

        Self { db, redis, rabbitmq }
    }
}
