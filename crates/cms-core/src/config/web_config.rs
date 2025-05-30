use serde::Deserialize;
use tracing::{Level, warn};

#[derive(Deserialize, Debug, Default)]
pub struct WebConfig {
    name: Option<String>,
    host: Option<String>,
    port: Option<u16>,
    version: Option<String>,
    log_level: Option<String>,
    description: Option<String>,
}

impl WebConfig {
    pub fn from_env() -> Result<Self, envy::Error> {
        let config = envy::prefixed("CMS_WEB_").from_env::<WebConfig>()?;
        let host = config.host.as_deref().unwrap_or_default();
        let port = config.port.unwrap_or_default();

        if host.is_empty() || port < 1000 {
            return Err(envy::Error::Custom(
                "Missing or invalid 'host' or 'port' configuration.".to_string(),
            ));
        }

        Ok(config)
    }

    fn get_default(value: &Option<String>, default: &str) -> String {
        let val = value.as_deref().unwrap_or_default();
        if val.is_empty() {
            default.to_string()
        } else {
            val.to_string()
        }
    }

    pub fn address(&self) -> String {
        format!(
            "{}:{}",
            Self::get_default(&self.host, "localhost"),
            self.port.unwrap(),
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
    use std::env;

    use super::*;

    #[test]
    fn test_web_config_from_env() {
        let config = WebConfig::from_env().unwrap();

        let name = env::var("CMS_WEB_NAME");
        if name.is_ok() {
            assert_eq!(config.name.unwrap(), name.unwrap());
        } else {
            assert!(config.name.is_none());
        }

        let host = env::var("CMS_WEB_HOST");
        if host.is_ok() {
            assert_eq!(config.host.unwrap(), host.unwrap());
        } else {
            assert!(config.host.is_none());
        }

        let port = env::var("CMS_WEB_PORT");
        if port.is_ok() {
            assert_eq!(config.port.unwrap(), port.unwrap().parse::<u16>().unwrap());
        } else {
            assert!(config.port.is_none());
        }

        let version = env::var("CMS_WEB_VERSION");
        if version.is_ok() {
            assert_eq!(config.version.unwrap(), version.unwrap());
        } else {
            assert!(config.version.is_none());
        }

        let log_level = env::var("CMS_WEB_LOG_LEVEL");
        if log_level.is_ok() {
            assert_eq!(config.log_level.unwrap(), log_level.unwrap());
        } else {
            assert!(config.log_level.is_none());
        }

        let description = env::var("CMS_WEB_DESCRIPTION");
        if description.is_ok() {
            assert_eq!(config.description.unwrap(), description.unwrap());
        } else {
            assert!(config.description.is_none());
        }
    }

    #[test]
    fn test_web_config_address() {
        let mut config = WebConfig::from_env().unwrap();

        let host = "192.168.1.2";
        let port = 3456;
        let fmt_address = format!("{}:{}", host, port);
        config.host = Some(host.to_string());
        config.port = Some(port);
        assert_eq!(config.address(), fmt_address);

        config.host = None;
        assert_eq!(config.address(), format!("localhost:{}", port));
        config.host = Some("".to_string());
        assert_eq!(config.address(), format!("localhost:{}", port));
        config.host = Some(host.to_string());

        config.port = Some(3100);
        assert_eq!(config.address(), format!("{}:3100", host));
        config.port = Some(3200);
        assert_eq!(config.address(), format!("{}:3200", host));
        config.port = Some(port);
    }

    #[test]
    fn test_web_config_app_name() {
        let mut config = WebConfig::from_env().unwrap();
        let name = "CMS Test";
        let default_string = "Simple CMS".to_string();

        config.name = None;
        assert_eq!(config.app_name(), default_string);
        config.name = Some("".to_string());
        assert_eq!(config.app_name(), default_string);
        config.name = Some(name.to_string());
        assert_eq!(config.app_name(), name.to_string());
    }

    #[test]
    fn test_web_config_app_version() {
        let mut config = WebConfig::from_env().unwrap();
        let version = "1.2.3";
        let default_string = "0.0.1".to_string();

        config.version = None;
        assert_eq!(config.app_version(), default_string);
        config.version = Some("".to_string());
        assert_eq!(config.app_version(), default_string);
        config.version = Some(version.to_string());
        assert_eq!(config.app_version(), version.to_string());
    }

    #[test]
    fn test_web_config_app_description() {
        let mut config = WebConfig::from_env().unwrap();
        let description = "CMS for simple websites";
        let default_string = "A simple CMS".to_string();

        config.description = None;
        assert_eq!(config.app_description(), default_string);
        config.description = Some("".to_string());
        assert_eq!(config.app_description(), default_string);
        config.description = Some(description.to_string());
        assert_eq!(config.app_description(), description.to_string());
    }

    #[test]
    fn test_web_config_tracing_level() {
        let mut config = WebConfig::from_env().unwrap();

        config.log_level = None;
        assert_eq!(config.tracing_level(), Level::INFO);
        config.log_level = Some("".to_string());
        assert_eq!(config.tracing_level(), Level::INFO);
        config.log_level = Some("info".to_string());
        assert_eq!(config.tracing_level(), Level::INFO);
        config.log_level = Some("debug".to_string());
        assert_eq!(config.tracing_level(), Level::DEBUG);
    }
}
