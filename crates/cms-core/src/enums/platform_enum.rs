use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

use super::SelectValueEnum;
use crate::domain::model::SelectOptionModel;

const OPEN_TITLE: &str = "用户端";
const MANAGER_TITLE: &str = "管理端";
const SYSTEM_TITLE: &str = "系统端";
const OPEN_VALUE: &str = "open";
const MANAGER_VALUE: &str = "manager";
const SYSTEM_VALUE: &str = "system";

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, ToSchema)]
pub enum PlatformEnum {
    Open,
    Manager,
    System,
}

impl PlatformEnum {
    pub fn as_value(&self) -> &'static str {
        match self {
            PlatformEnum::Open => OPEN_VALUE,
            PlatformEnum::Manager => MANAGER_VALUE,
            PlatformEnum::System => SYSTEM_VALUE,
        }
    }

    pub fn as_title(&self) -> &'static str {
        match self {
            PlatformEnum::Open => OPEN_TITLE,
            PlatformEnum::Manager => MANAGER_TITLE,
            PlatformEnum::System => SYSTEM_TITLE,
        }
    }

    pub fn is_manager(&self) -> bool {
        match self {
            PlatformEnum::Manager => true,
            _ => false,
        }
    }

    pub fn form_string(value: String) -> Self {
        let str = value.to_lowercase();
        match str.as_str() {
            "open" => PlatformEnum::Open,
            "manager" => PlatformEnum::Manager,
            _ => PlatformEnum::System,
        }
    }
}

impl Into<SelectOptionModel> for PlatformEnum {
    fn into(self) -> SelectOptionModel {
        let value = self.as_value();
        SelectOptionModel {
            label: self.as_title().to_string(),
            value: SelectValueEnum::Str(value),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_value() {
        assert_eq!(PlatformEnum::Open.as_value(), "open");
        assert_eq!(PlatformEnum::Manager.as_value(), "manager");
        assert_eq!(PlatformEnum::System.as_value(), "system");
    }

    #[test]
    fn test_as_title() {
        assert_eq!(PlatformEnum::Open.as_title(), "用户端");
        assert_eq!(PlatformEnum::Manager.as_title(), "管理端");
        assert_eq!(PlatformEnum::System.as_title(), "系统端");
    }

    #[test]
    fn test_is_manager() {
        assert!(!PlatformEnum::Open.is_manager());
        assert!(PlatformEnum::Manager.is_manager());
        assert!(!PlatformEnum::System.is_manager());
    }

    #[test]
    fn test_form_string() {
        assert_eq!(PlatformEnum::form_string("open".to_string()), PlatformEnum::Open);
        assert_eq!(PlatformEnum::form_string("OPEN".to_string()), PlatformEnum::Open);
        assert_eq!(PlatformEnum::form_string("OpEn".to_string()), PlatformEnum::Open);

        assert_eq!(PlatformEnum::form_string("manager".to_string()), PlatformEnum::Manager);
        assert_eq!(PlatformEnum::form_string("MANAGER".to_string()), PlatformEnum::Manager);
        assert_eq!(PlatformEnum::form_string("mAnAgEr".to_string()), PlatformEnum::Manager);

        assert_eq!(PlatformEnum::form_string("system".to_string()), PlatformEnum::System);
        assert_eq!(PlatformEnum::form_string("sys".to_string()), PlatformEnum::System);
        assert_eq!(PlatformEnum::form_string("".to_string()), PlatformEnum::System);
        assert_eq!(PlatformEnum::form_string("other".to_string()), PlatformEnum::System);
    }

    #[test]
    fn test_into_select_option_model() {
        let option: SelectOptionModel = PlatformEnum::Open.into();
        assert_eq!(option.label, "用户端");
        assert_eq!(option.value, SelectValueEnum::Str("open"));

        let option: SelectOptionModel = PlatformEnum::Manager.into();
        assert_eq!(option.label, "管理端");
        assert_eq!(option.value, SelectValueEnum::Str("manager"));

        let option: SelectOptionModel = PlatformEnum::System.into();
        assert_eq!(option.label, "系统端");
        assert_eq!(option.value, SelectValueEnum::Str("system"));
    }
}