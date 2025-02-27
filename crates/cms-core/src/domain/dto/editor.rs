use serde::{Deserialize, Serialize};

use super::JwtClaimsDTO;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct EditorCurrent {
    pub editor_id: i64,
    pub editor_type: String,
}

impl EditorCurrent {
    fn from_claims(claims: &JwtClaimsDTO) -> Self {
        Self {
            editor_id: claims.user_id,
            editor_type: claims.user_type.clone(),
        }
    }

    pub fn empty() -> Self {
        Self {
            editor_id: 0,
            editor_type: String::from("system"),
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
