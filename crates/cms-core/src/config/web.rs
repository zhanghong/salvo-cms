use dotenvy::dotenv;
use serde::Deserialize;
use tracing::{Level, info, warn};

#[derive(Deserialize, Debug)]
pub struct WebConfig {
    name: Option<String>,
    host: Option<String>,
    port: Option<u16>,
    api_prefix: Option<String>,
    version: Option<String>,
    log_level: Option<String>,
    description: Option<String>,
    swagger_path: Option<String>,
    openapi_path: Option<String>,
}

impl WebConfig {
    pub fn from_env() -> Result<Self, envy::Error> {
        match dotenv() {
            Ok(_) => info!("Loaded environment variables from .env file."),
            Err(e) => warn!("Failed to load .env file: {}", e),
        }
        let config = envy::prefixed("CMS_WEB_").from_env::<WebConfig>()?;

        // 验证关键字段
        if config.host.as_deref() == Some("") || config.port.is_none() {
            return Err(envy::Error::Custom(
                "Missing or invalid 'host' or 'port' configuration.".to_string(),
            ));
        }

        Ok(config)
    }

    fn get_default(value: &Option<String>, default: &str) -> String {
        value.as_deref().unwrap_or(default).to_string()
    }

    pub fn address(&self) -> String {
        format!(
            "{}:{}",
            Self::get_default(&self.host, "localhost"),
            self.port.unwrap_or(3000),
        )
    }

    pub fn app_name(&self) -> String {
        Self::get_default(&self.name, "Simple CMS")
    }

    pub fn app_version(&self) -> String {
        Self::get_default(&self.version, "0.0.1")
    }

    pub fn app_description(&self) -> String {
        Self::get_default(&self.description, "A simple CMS")
    }

    pub fn app_api_prefix(&self) -> String {
        Self::get_default(&self.api_prefix, "")
    }

    pub fn swagger_url(&self) -> String {
        Self::get_default(&self.swagger_path, "/swagger-ui")
    }

    pub fn openapi_url(&self) -> String {
        Self::get_default(&self.openapi_path, "/api-docs/openapi.json")
    }

    pub fn tracing_level(&self) -> Level {
        let name = self.log_level.as_deref().unwrap_or("info");

        match name.to_lowercase().as_str() {
            "debug" => Level::DEBUG,
            "warn" => Level::WARN,
            "error" => Level::ERROR,
            "info" => Level::INFO,
            _ => {
                warn!("Invalid log level '{}', falling back to 'info'", name);
                Level::INFO
            }
        }
    }
}
