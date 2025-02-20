use redis::Client;
use sea_orm::DatabaseConnection;

use super::DbConfig;
use super::RedisConfig;

#[derive(Clone, Debug)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub redis: Client,
}

impl AppState {
    pub async fn init() -> Self {
        let db_config = DbConfig::from_env().expect("Failed to load db config");
        let redis_config = RedisConfig::from_env().expect("Failed to load redis config");

        let db = db_config.build_connection().await.unwrap();
        let redis = redis_config.build_client().await.unwrap();

        Self { db, redis }
    }
}
