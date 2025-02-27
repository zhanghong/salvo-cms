use serde::{Deserialize, Serialize};

use super::JwtClaimsDTO;
use crate::enums::EditorTypeEnum;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct EditorCurrent {
    pub editor_id: i64,
    pub editor_type: EditorTypeEnum,
}

impl EditorCurrent {
    fn from_claims(claims: &JwtClaimsDTO) -> Self {
        Self {
            editor_id: claims.user_id,
            editor_type: EditorTypeEnum::from_string(claims.user_type.as_str()),
        }
    }

    pub fn empty() -> Self {
        Self {
            editor_id: 0,
            editor_type: EditorTypeEnum::None,
        }
    }
}

impl From<JwtClaimsDTO> for EditorCurrent {
    fn from(claims: JwtClaimsDTO) -> Self {
        Self::from_claims(&claims)
    }
}

impl From<&JwtClaimsDTO> for EditorCurrent {
    fn from(claims: &JwtClaimsDTO) -> Self {
        Self::from_claims(claims)
    }
}
