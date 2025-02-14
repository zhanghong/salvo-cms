use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

use crate::domain::{SelectOptionItem, SelectValueEnum};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, ToSchema)]
pub enum PlatformEnum {
    Open,
    Manager,
    System,
}

impl PlatformEnum {
    pub fn as_value(&self) -> String {
        match self {
            PlatformEnum::Open => String::from("open"),
            PlatformEnum::Manager => String::from("manager"),
            PlatformEnum::System => String::from("system"),
        }
    }

    pub fn as_title(&self) -> String {
        match self {
            PlatformEnum::Open => String::from("用户端"),
            PlatformEnum::Manager => String::from("管理端"),
            PlatformEnum::System => String::from("系统端"),
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

impl Into<SelectOptionItem> for PlatformEnum {
    fn into(self) -> SelectOptionItem {
        let value = self.as_value();
        SelectOptionItem {
            label: self.as_title(),
            value: SelectValueEnum::String(value),
            ..Default::default()
        }
    }
}
