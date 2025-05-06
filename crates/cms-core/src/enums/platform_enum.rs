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
