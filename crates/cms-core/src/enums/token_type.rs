use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

const ACCESS_TOKEN_TITLE: &str = "Access Token";
const REFRESH_TOKEN_TITLE: &str = "Refresh Token";
const NONE_TITLE: &str = "Refresh Token";

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, ToSchema)]
pub enum TokenTypeEnum {
    None,
    AccessToken,
    RefreshToken,
}

impl TokenTypeEnum {
    pub fn as_value(&self) -> String {
        match self {
            TokenTypeEnum::AccessToken => String::from("access_token"),
            TokenTypeEnum::RefreshToken => String::from("refresh_token"),
            _ => String::from("none"),
        }
    }

    pub fn as_title(&self) -> &'static str {
        match self {
            TokenTypeEnum::AccessToken => ACCESS_TOKEN_TITLE,
            TokenTypeEnum::RefreshToken => REFRESH_TOKEN_TITLE,
            _ => NONE_TITLE,
        }
    }

    pub fn form_string(value: String) -> Self {
        let str = value.to_lowercase();
        match str.as_str() {
            "access_token" => TokenTypeEnum::AccessToken,
            "refresh_token" => TokenTypeEnum::RefreshToken,
            _ => TokenTypeEnum::None,
        }
    }
}
