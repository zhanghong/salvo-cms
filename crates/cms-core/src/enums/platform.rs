use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, ToSchema)]
pub enum PlatformEnum {
    User,
    Manager,
    System,
}

impl PlatformEnum {
    pub fn value(&self) -> String {
        match self {
            PlatformEnum::User => String::from("user"),
            PlatformEnum::Manager => String::from("manager"),
            PlatformEnum::System => String::from("system"),
        }
    }

    pub fn title(&self) -> String {
        match self {
            PlatformEnum::User => String::from("用户端"),
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
            "user" => PlatformEnum::User,
            "manager" => PlatformEnum::Manager,
            _ => PlatformEnum::System,
        }
    }
}
