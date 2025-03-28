use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

use crate::domain::{SelectOptionItem, SelectValueEnum};

const OPEN_TITLE: &str = "用户端";
const MAMAGER_TITLE: &str = "管理端";
const SYSTEM_TITLE: &str = "系统端";

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, ToSchema)]
pub enum PlatformEnum {
    Open,
    Manager,
    System,
}

impl PlatformEnum {
    pub fn as_value(&self) -> String {
        match self {
            PlatformEnum::Open => "open",
            PlatformEnum::Manager => "manager",
            PlatformEnum::System => "system",
        }
        .to_string()
    }

    pub fn as_title(&self) -> &'static str {
        match self {
            PlatformEnum::Open => OPEN_TITLE,
            PlatformEnum::Manager => MAMAGER_TITLE,
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

impl Into<SelectOptionItem> for PlatformEnum {
    fn into(self) -> SelectOptionItem {
        let value = self.as_value();
        SelectOptionItem {
            label: self.as_title().to_string(),
            value: SelectValueEnum::String(value),
            ..Default::default()
        }
    }
}
