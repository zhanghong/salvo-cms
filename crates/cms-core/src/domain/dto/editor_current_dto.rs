use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::JwtClaimsDTO;
use crate::enums::EditorTypeEnum;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct EditorCurrentDTO {
    pub editor_id: Option<Uuid>,
    pub editor_type: EditorTypeEnum,
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
        let uuid = Uuid::parse_str(&claims.user_id).unwrap();
        Self {
            editor_id: Some(uuid),
            editor_type,
        }
    }

    pub fn empty() -> Self {
        Self {
            editor_id: None,
            editor_type: EditorTypeEnum::None,
        }
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
