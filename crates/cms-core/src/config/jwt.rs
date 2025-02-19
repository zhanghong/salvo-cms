use dotenvy::dotenv;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct JwtConfig {
    access_secret: Option<String>,
    refresh_secret: Option<String>,
    access_expire_days: Option<i64>,
    refresh_expire_days: Option<i64>,
}

impl JwtConfig {
    pub fn from_env() -> Result<Self, envy::Error> {
        dotenv().ok();
        envy::prefixed("JWT_").from_env::<JwtConfig>()
    }

    pub fn access_secret_bytes(&self) -> Vec<u8> {
        let str = self
            .access_secret
            .clone()
            .unwrap_or("Cms Access Token Secret".to_string());
        str.into_bytes()
    }

    pub fn refresh_secret_bytes(&self) -> Vec<u8> {
        let str = self
            .refresh_secret
            .clone()
            .unwrap_or("Cms Refresh Token Secret".to_string());
        str.into_bytes()
    }

    pub fn get_access_expire_days(&self) -> i64 {
        self.access_expire_days.unwrap_or(7)
    }

    pub fn get_refresh_expire_days(&self) -> i64 {
        self.refresh_expire_days.unwrap_or(365)
    }
}
