use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::JwtClaimsDTO;
use crate::enums::EditorTypeEnum;

/// Current Editor DTO
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct EditorCurrentDTO {
    pub editor_id: Uuid,
    pub editor_type: EditorTypeEnum,
}

impl Default for EditorCurrentDTO {
    fn default() -> Self {
        Self {
            editor_id: Uuid::nil(),
            editor_type: EditorTypeEnum::None,
        }
    }
}

impl EditorCurrentDTO {
    fn from_claims(claims: &JwtClaimsDTO) -> Self {
        let user_types = claims.user_type.to_owned();
        let user_types = user_types.to_lowercase();
        let editor_type = match user_types.as_str() {
            "manager" => EditorTypeEnum::Admin,
            "open" => EditorTypeEnum::Member,
            _ => EditorTypeEnum::None,
        };
        let res = Uuid::parse_str(&claims.user_id);
        let uuid = match res {
            Ok(uuid) => uuid,
            Err(_) => Uuid::nil(),
        };
        Self {
            editor_id: uuid,
            editor_type,
        }
    }

    pub fn empty() -> Self {
        Self::default()
    }
}

impl From<JwtClaimsDTO> for EditorCurrentDTO {
    fn from(claims: JwtClaimsDTO) -> Self {
        Self::from_claims(&claims)
    }
}

impl From<&JwtClaimsDTO> for EditorCurrentDTO {
    fn from(claims: &JwtClaimsDTO) -> Self {
        Self::from_claims(claims)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use uuid::Uuid;

    use crate::domain::dto::JwtClaimsDTO;
    use crate::enums::EditorTypeEnum;

    #[test]
    fn test_from_claims_with_manager() {
        let user_id = Uuid::new_v4().to_string();
        let mut claims = JwtClaimsDTO {
            user_id: user_id.clone(),
            user_type: "Manager".to_string(),
            ..Default::default()
        };

        let dto = EditorCurrentDTO::from_claims(&claims);
        assert_eq!(dto.editor_type, EditorTypeEnum::Admin);
        assert_eq!(dto.editor_id, Uuid::parse_str(&user_id).unwrap());

        claims.user_type = "manager".to_string();
        let dto = EditorCurrentDTO::from_claims(&claims);
        assert_eq!(dto.editor_type, EditorTypeEnum::Admin);
    }

    #[test]
    fn test_from_claims_with_open() {
        let user_id = Uuid::new_v4().to_string();
        let mut claims = JwtClaimsDTO {
            user_id: user_id.clone(),
            user_type: "Open".to_string(),
            ..Default::default()
        };

        let dto = EditorCurrentDTO::from_claims(&claims);
        assert_eq!(dto.editor_type, EditorTypeEnum::Member);
        assert_eq!(dto.editor_id, Uuid::parse_str(&user_id).unwrap());

        claims.user_type = "open".to_string();
        let dto = EditorCurrentDTO::from_claims(&claims);
        assert_eq!(dto.editor_type, EditorTypeEnum::Member);
        assert_eq!(dto.editor_id, Uuid::parse_str(&user_id).unwrap());
    }

    #[test]
    fn test_from_claims_with_invalid_user_type() {
        let user_id = Uuid::new_v4().to_string();
        let mut claims = JwtClaimsDTO {
            user_id: user_id.clone(),
            user_type: "invalid".to_string(),
            ..Default::default()
        };

        let dto = EditorCurrentDTO::from_claims(&claims);
        assert_eq!(dto.editor_type, EditorTypeEnum::None);
        assert_eq!(dto.editor_id, Uuid::parse_str(&user_id).unwrap());

        claims.user_type = "".to_string();
        let dto = EditorCurrentDTO::from_claims(&claims);
        assert_eq!(dto.editor_type, EditorTypeEnum::None);
    }

    #[test]
    fn test_from_claims_with_invalid_uuid() {
        let claims = JwtClaimsDTO {
            user_id: "not-a-uuid".to_string(),
            user_type: "Manager".to_string(),
            ..Default::default()
        };

        let dto = EditorCurrentDTO::from(claims);
        assert_eq!(dto.editor_type, EditorTypeEnum::Admin);
        assert_eq!(dto.editor_id, Uuid::nil());
    }

    #[test]
    fn test_empty_returns_none_editor_type_and_no_id() {
        let dto = EditorCurrentDTO::empty();
        assert_eq!(dto.editor_type, EditorTypeEnum::None);
        assert_eq!(dto.editor_id, Uuid::nil());
    }

    #[test]
    fn test_from_jwt_claims_for_consumed_input() {
        let user_id = Uuid::new_v4().to_string();
        let claims = JwtClaimsDTO {
            user_id: user_id.clone(),
            user_type: "Manager".to_string(),
            ..Default::default()
        };

        let dto = EditorCurrentDTO::from(claims);
        assert_eq!(dto.editor_type, EditorTypeEnum::Admin);
        assert_eq!(dto.editor_id, Uuid::parse_str(&user_id).unwrap());
    }

    #[test]
    fn test_from_jwt_claims_for_borrowed_input() {
        let user_id = Uuid::new_v4().to_string();
        let claims = JwtClaimsDTO {
            user_id: user_id.clone(),
            user_type: "Manager".to_string(),
            ..Default::default()
        };

        let dto = EditorCurrentDTO::from(&claims);
        assert_eq!(dto.editor_type, EditorTypeEnum::Admin);
        assert_eq!(dto.editor_id, Uuid::parse_str(&user_id).unwrap());
    }

    #[test]
    fn test_default_initialization() {
        let dto = EditorCurrentDTO::default();
        assert_eq!(dto.editor_id, Uuid::nil());
        assert_eq!(dto.editor_type, EditorTypeEnum::None);
    }

    #[test]
    fn test_function_empty() {
        let dto = EditorCurrentDTO::empty();
        assert_eq!(dto.editor_id, Uuid::nil());
        assert_eq!(dto.editor_type, EditorTypeEnum::None);
    }
}
