use dotenvy::dotenv;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct JwtConfig {
    secret_key: Option<String>,
    expire_days: Option<u16>,
}

impl JwtConfig {
    pub fn from_env() -> Result<Self, envy::Error> {
        dotenv().ok();
        envy::prefixed("JWT_").from_env::<JwtConfig>()
    }

    pub fn secret_bytes(&self) -> Vec<u8> {
        let str = self.secret_key.clone().unwrap_or("FoxCms".to_string());
        str.into_bytes()
    }

    pub fn expired_days(&self) -> u16 {
        self.expire_days.unwrap_or(7)
    }
}
