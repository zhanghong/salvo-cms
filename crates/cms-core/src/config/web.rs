use dotenvy::dotenv;
use serde::Deserialize;
use tracing::Level;

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
        dotenv().ok();
        envy::prefixed("CMS_WEB_").from_env::<WebConfig>()
    }

    pub fn address(&self) -> String {
        format!(
            "{}:{}",
            self.host.as_ref().unwrap_or(&"localhost".to_string()),
            self.port.unwrap_or(3000),
        )
    }

    pub fn app_name(&self) -> String {
        format!(
            "{}",
            self.name.as_ref().unwrap_or(&String::from("Simple CMS"))
        )
    }

    pub fn app_version(&self) -> String {
        format!(
            "{}",
            self.version.as_ref().unwrap_or(&String::from("0.0.1"))
        )
    }

    pub fn app_description(&self) -> String {
        format!(
            "{}",
            self.description
                .as_ref()
                .unwrap_or(&String::from("A simple CMS"))
        )
    }

    pub fn app_api_prefix(&self) -> String {
        format!("{}", self.api_prefix.as_ref().unwrap_or(&String::from("")))
    }

    pub fn swagger_url(&self) -> String {
        format!(
            "{}",
            self.swagger_path
                .as_ref()
                .unwrap_or(&String::from("/swagger-ui"))
        )
    }

    pub fn openapi_url(&self) -> String {
        format!(
            "{}",
            self.openapi_path
                .as_ref()
                .unwrap_or(&String::from("/api-docs/openapi.json"))
        )
    }

    pub fn tracing_level(&self) -> Level {
        let name = match self.log_level.as_ref() {
            Some(name) => name.as_str(),
            None => "info",
        };

        match name {
            "debug" => Level::DEBUG,
            "warn" => Level::WARN,
            "error" => Level::ERROR,
            _ => Level::INFO,
        }
    }
}
