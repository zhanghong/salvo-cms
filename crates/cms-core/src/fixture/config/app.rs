use sea_orm::DatabaseBackend;
use sea_orm::MockDatabase;

use crate::config::AppState;
use crate::config::RabbitMQConfig;
use crate::config::RedisConfig;

pub struct FakerAppState {}
impl FakerAppState {
    pub async fn init() -> AppState {
        let db = MockDatabase::new(DatabaseBackend::Postgres).into_connection();
        let redis_config = RedisConfig::from_env().expect("Failed to load redis config");
        let rabbitmq_config = RabbitMQConfig::from_env().expect("Failed to load queue config");

        let redis = redis_config.build_client().await.unwrap();
        let rabbitmq = rabbitmq_config.build_pool().await.unwrap();

        AppState {
            db,
            redis,
            rabbitmq,
        }
    }
}
