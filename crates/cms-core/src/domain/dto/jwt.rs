use serde::{Deserialize, Serialize};

use crate::enums::EditorTypeEnum;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct JwtClaimsDTO {
    pub uuid: String,
    pub editor_id: i64,
    pub editor_type: EditorTypeEnum,
    pub token_type: String,
    pub exp: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct JwtTokenDTO {
    pub token_type: String,
    pub token_value: String,
    pub expired_time: i64,
}
