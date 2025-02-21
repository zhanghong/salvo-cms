use dotenvy::dotenv;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct JwtConfig {
    secret_key: Option<String>,
    access_expire_days: Option<i64>,
    refresh_expire_days: Option<i64>,
}

impl JwtConfig {
    pub fn from_env() -> Result<Self, envy::Error> {
        dotenv().ok();
        envy::prefixed("JWT_").from_env::<JwtConfig>()
    }

    pub fn secret_bytes(&self) -> Vec<u8> {
        let str = self
            .secret_key
            .clone()
            .unwrap_or("Cms Jwt Secret Key".to_string());
        str.into_bytes()
    }

    pub fn get_access_expire_days(&self) -> i64 {
        self.access_expire_days.unwrap_or(7)
    }

    pub fn get_refresh_expire_days(&self) -> i64 {
        self.refresh_expire_days.unwrap_or(365)
    }
}
