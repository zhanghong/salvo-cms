use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

use crate::consts::enum_consts::*;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, ToSchema)]
pub enum TokenTypeEnum {
    None,
    AccessToken,
    RefreshToken,
}

impl TokenTypeEnum {
    pub fn as_value(&self) -> String {
        match self {
            TokenTypeEnum::AccessToken => String::from(TOKEN_TYPE_ACCESS_VALUE),
            TokenTypeEnum::RefreshToken => String::from(TOKEN_TYPE_REFRESH_VALUE),
            _ => String::from(TOKEN_TYPE_NONE_VALUE),
        }
    }

    pub fn as_title(&self) -> &'static str {
        match self {
            TokenTypeEnum::AccessToken => TOKEN_TYPE_ACCESS_TITLE,
            TokenTypeEnum::RefreshToken => TOKEN_TYPE_REFRESH_TITLE,
            _ => TOKEN_TYPE_NONE_TITLE,
        }
    }

    pub fn form_string(value: String) -> Self {
        let str = value.to_lowercase();
        match str.as_str() {
            TOKEN_TYPE_ACCESS_VALUE => TokenTypeEnum::AccessToken,
            TOKEN_TYPE_REFRESH_VALUE => TokenTypeEnum::RefreshToken,
            _ => TokenTypeEnum::None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TokenTypeEnum;
    use crate::consts::enum_consts::*;

    #[test]
    fn test_as_value() {
        assert_eq!(
            TokenTypeEnum::AccessToken.as_value(),
            TOKEN_TYPE_ACCESS_VALUE
        );
        assert_eq!(
            TokenTypeEnum::RefreshToken.as_value(),
            TOKEN_TYPE_REFRESH_VALUE
        );
        assert_eq!(TokenTypeEnum::None.as_value(), TOKEN_TYPE_NONE_VALUE);
    }

    #[test]
    fn test_as_title() {
        assert_eq!(
            TokenTypeEnum::AccessToken.as_title(),
            TOKEN_TYPE_ACCESS_TITLE
        );
        assert_eq!(
            TokenTypeEnum::RefreshToken.as_title(),
            TOKEN_TYPE_REFRESH_TITLE
        );
        assert_eq!(TokenTypeEnum::None.as_title(), TOKEN_TYPE_NONE_TITLE);
    }

    #[test]
    fn test_form_string() {
        assert_eq!(
            TokenTypeEnum::form_string("access_token".to_string()),
            TokenTypeEnum::AccessToken
        );
        assert_eq!(
            TokenTypeEnum::form_string("ACCESS_TOKEN".to_string()),
            TokenTypeEnum::AccessToken
        );
        assert_eq!(
            TokenTypeEnum::form_string("refresh_token".to_string()),
            TokenTypeEnum::RefreshToken
        );
        assert_eq!(
            TokenTypeEnum::form_string("REFRESH_TOKEN".to_string()),
            TokenTypeEnum::RefreshToken
        );
        assert_eq!(
            TokenTypeEnum::form_string("invalid".to_string()),
            TokenTypeEnum::None
        );
        assert_eq!(
            TokenTypeEnum::form_string("".to_string()),
            TokenTypeEnum::None
        );
    }

    // #[test]
    // fn test_serde_serialize_deserialize() {
    //     use serde_json;

    //     let token = TokenTypeEnum::AccessToken;
    //     let serialized = serde_json::to_string(&token).unwrap();
    //     assert_eq!(serialized, "\"access_token\"");

    //     let deserialized: TokenTypeEnum = serde_json::from_str("\"access_token\"").unwrap();
    //     assert_eq!(deserialized, TokenTypeEnum::AccessToken);

    //     let deserialized_invalid: TokenTypeEnum =
    //         serde_json::from_str("\"invalid\"").unwrap_or(TokenTypeEnum::None);
    //     assert_eq!(deserialized_invalid, TokenTypeEnum::None);
    // }
}
