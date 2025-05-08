use dotenvy::dotenv;
use serde::Deserialize;
use tracing::{Level, info, warn};

#[derive(Deserialize, Debug, Default)]
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

// ... existing code ...

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    fn setup_env(vars: &[(&str, &str)]) {
        unsafe {
            for (key, value) in vars {
                env::set_var(key, value);
            }
        }
    }

    fn clear_env_vars() {
        let keys = [
            "CMS_WEB_NAME",
            "CMS_WEB_HOST",
            "CMS_WEB_PORT",
            "CMS_WEB_API_PREFIX",
            "CMS_WEB_VERSION",
            "CMS_WEB_LOG_LEVEL",
            "CMS_WEB_DESCRIPTION",
            "CMS_WEB_SWAGGER_PATH",
            "CMS_WEB_OPENAPI_PATH",
        ];
        unsafe {
            for key in keys {
                env::remove_var(key);
            }
        }
    }

    #[test]
    fn test_web_config_from_env() {
        clear_env_vars();
        setup_env(&[
            ("CMS_WEB_HOST", "127.0.0.1"),
            ("CMS_WEB_PORT", "5800"),
            ("CMS_WEB_NAME", "Test CMS"),
            ("CMS_WEB_VERSION", "1.0.0"),
            ("CMS_WEB_DESCRIPTION", "Test Description"),
            ("CMS_WEB_API_PREFIX", "/api"),
            ("CMS_WEB_LOG_LEVEL", "debug"),
            ("CMS_WEB_SWAGGER_PATH", "/docs"),
            ("CMS_WEB_OPENAPI_PATH", "/openapi.json"),
        ]);

        let config = WebConfig::from_env().expect("Failed to load config");

        assert_eq!(config.host.as_deref().unwrap(), "127.0.0.1");
        assert_eq!(config.port.unwrap(), 5800);
        assert_eq!(config.name.as_deref().unwrap(), "Test CMS");
        assert_eq!(config.version.as_deref().unwrap(), "1.0.0");
        assert_eq!(config.description.as_deref().unwrap(), "Test Description");
        assert_eq!(config.api_prefix.as_deref().unwrap(), "/api");
        assert_eq!(config.log_level.as_deref().unwrap(), "debug");
        assert_eq!(config.swagger_path.as_deref().unwrap(), "/docs");
        assert_eq!(config.openapi_path.as_deref().unwrap(), "/openapi.json");
    }

    #[test]
    fn test_web_config_address() {
        let config = WebConfig {
            host: Some("127.0.0.1".to_string()),
            port: Some(5800),
            ..Default::default()
        };
        assert_eq!(config.address(), "127.0.0.1:5800");

        let config = WebConfig {
            host: None,
            port: None,
            ..Default::default()
        };
        assert_eq!(config.address(), "localhost:3000");
    }

    #[test]
    fn test_get_default_with_some() {
        let value = Some("custom".to_string());
        assert_eq!(WebConfig::get_default(&value, "default"), "custom");
    }

    #[test]
    fn test_get_default_with_none() {
        let value: Option<String> = None;
        assert_eq!(WebConfig::get_default(&value, "default"), "default");
    }

    #[test]
    fn test_address_with_values() {
        let config = WebConfig {
            host: Some("example.com".to_string()),
            port: Some(8080),
            ..Default::default()
        };
        assert_eq!(config.address(), "example.com:8080");
    }

    #[test]
    fn test_address_with_defaults() {
        let config = WebConfig {
            host: None,
            port: None,
            ..Default::default()
        };
        assert_eq!(config.address(), "localhost:3000");
    }

    #[test]
    fn test_app_name_with_value() {
        let config = WebConfig {
            name: Some("MyApp".to_string()),
            ..Default::default()
        };
        assert_eq!(config.app_name(), "MyApp");
    }

    #[test]
    fn test_app_name_with_default() {
        let config = WebConfig {
            name: None,
            ..Default::default()
        };
        assert_eq!(config.app_name(), "Simple CMS");
    }

    #[test]
    fn test_app_version_with_value() {
        let config = WebConfig {
            version: Some("v1.0.0".to_string()),
            ..Default::default()
        };
        assert_eq!(config.app_version(), "v1.0.0");
    }

    #[test]
    fn test_app_version_with_default() {
        let config = WebConfig {
            version: None,
            ..Default::default()
        };
        assert_eq!(config.app_version(), "0.0.1");
    }

    #[test]
    fn test_app_description_with_value() {
        let config = WebConfig {
            description: Some("My custom CMS".to_string()),
            ..Default::default()
        };
        assert_eq!(config.app_description(), "My custom CMS");
    }

    #[test]
    fn test_app_description_with_default() {
        let config = WebConfig {
            description: None,
            ..Default::default()
        };
        assert_eq!(config.app_description(), "A simple CMS");
    }

    #[test]
    fn test_app_api_prefix_with_value() {
        let config = WebConfig {
            api_prefix: Some("/api/v1".to_string()),
            ..Default::default()
        };
        assert_eq!(config.app_api_prefix(), "/api/v1");
    }

    #[test]
    fn test_app_api_prefix_with_default() {
        let config = WebConfig {
            api_prefix: None,
            ..Default::default()
        };
        assert_eq!(config.app_api_prefix(), "");
    }

    #[test]
    fn test_swagger_url_with_value() {
        let config = WebConfig {
            swagger_path: Some("/docs/swagger".to_string()),
            ..Default::default()
        };
        assert_eq!(config.swagger_url(), "/docs/swagger");
    }

    #[test]
    fn test_swagger_url_with_default() {
        let config = WebConfig {
            swagger_path: None,
            ..Default::default()
        };
        assert_eq!(config.swagger_url(), "/swagger-ui");
    }

    #[test]
    fn test_openapi_url_with_value() {
        let config = WebConfig {
            openapi_path: Some("/docs/openapi.yaml".to_string()),
            ..Default::default()
        };
        assert_eq!(config.openapi_url(), "/docs/openapi.yaml");
    }

    #[test]
    fn test_openapi_url_with_default() {
        let config = WebConfig {
            openapi_path: None,
            ..Default::default()
        };
        assert_eq!(config.openapi_url(), "/api-docs/openapi.json");
    }

    #[test]
    fn test_web_config_tracing_level() {
        let test_cases = vec![
            ("debug", Level::DEBUG),
            ("info", Level::INFO),
            ("warn", Level::WARN),
            ("error", Level::ERROR),
            ("invalid", Level::INFO),
            ("", Level::INFO),
        ];

        for (input, expected) in test_cases {
            let config = WebConfig {
                log_level: Some(input.to_string()),
                ..Default::default()
            };
            assert_eq!(config.tracing_level(), expected);
        }
    }
}
