use sea_orm::DatabaseConnection;

use super::JwtConfig;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}
