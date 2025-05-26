use deadpool_lapin::{Config as LapinConfig, Pool as LapinPool, Runtime as LapinRuntime};
use redis::Client as RedisClient;
use sea_orm::DatabaseBackend;
use sea_orm::DatabaseConnection;
use sea_orm::MockDatabase;

use super::DatabaseConfig;
use super::RabbitMQConfig;
use super::RedisConfig;

#[derive(Debug)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub redis: RedisClient,
    pub rabbitmq: LapinPool,
}

impl AppState {
    pub async fn init() -> Self {
        let db_config = DatabaseConfig::from_env().expect("Failed to load db config");
        let redis_config = RedisConfig::from_env().expect("Failed to load redis config");
        let rabbitmq_config = RabbitMQConfig::from_env().expect("Failed to load queue config");

        let res = db_config.build_connection().await;
        println!("condition database: {:#?}", res);
        let db = db_config.build_connection().await.unwrap();
        let redis = redis_config.build_client().await.unwrap();
        let rabbitmq = rabbitmq_config.build_pool().await.unwrap();

        Self {
            db,
            redis,
            rabbitmq,
        }
    }

    pub fn test_init() -> Self {
        let db = MockDatabase::new(DatabaseBackend::Postgres).into_connection();
        let redis = RedisClient::open("redis://127.0.0.1:6379").unwrap();
        let rabbitmq = LapinConfig::default()
            .create_pool(Some(LapinRuntime::Tokio1))
            .unwrap();

        Self {
            db,
            redis,
            rabbitmq,
        }
    }
}

pub struct MockAppState {}
impl MockAppState {
    pub fn init() -> AppState {
        let db = MockDatabase::new(DatabaseBackend::Postgres).into_connection();
        let redis = RedisClient::open("redis://127.0.0.1:6379").unwrap();
        let rabbitmq = LapinConfig::default()
            .create_pool(Some(LapinRuntime::Tokio1))
            .unwrap();

        AppState {
            db,
            redis,
            rabbitmq,
        }
    }
}
