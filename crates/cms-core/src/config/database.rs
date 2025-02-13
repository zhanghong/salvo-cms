use dotenvy::dotenv;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DbConfig {
    pub protocol: Option<String>,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub user: String,
    pub password: String,
    pub name: String,
    pub schema: Option<String>,
    pub max_connections: Option<u32>,
    pub min_connections: Option<u32>,
    pub connect_timeout: Option<u64>,
    pub acquire_timeout: Option<u64>,
    pub idle_timeout: Option<u64>,
    pub max_lifetime: Option<u64>,
    pub sqlx_logging: Option<bool>,
}

impl DbConfig {
    pub fn from_env() -> Result<Self, envy::Error> {
        dotenv().ok();
        envy::prefixed("DB_").from_env::<DbConfig>()
    }

    pub fn url(&self) -> String {
        let prot = match self.protocol.as_ref() {
            Some(str) => str.as_str(),
            None => "mysql",
        };
        if "postgres".eq(prot) {
            format!(
                "postgres://{}:{}@{}:{}/{}?schema={}",
                self.user,
                self.password,
                self.host.as_ref().unwrap_or(&"localhost".to_string()),
                self.port.unwrap_or(5432),
                self.name,
                self.schema.as_ref().unwrap_or(&"public".to_string())
            )
        } else {
            format!(
                "mysql://{}:{}@{}:{}/{}",
                self.user,
                self.password,
                self.host.as_ref().unwrap_or(&"localhost".to_string()),
                self.port.unwrap_or(3306),
                self.name
            )
        }
    }
}
