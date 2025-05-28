use deadpool_lapin::{Config as LapinConfig, Runtime as LapinRuntime};
use redis::Client as RedisClient;
use sea_orm::DatabaseBackend;
use sea_orm::MockDatabase;

use crate::config::AppState;

pub struct FakerAppState {}
impl FakerAppState {
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
