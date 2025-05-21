use serde::{Deserialize, Serialize};

/// Jwt Token DTO
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct JwtTokenDTO {
    pub token_type: String,
    pub token_value: String,
    pub expired_time: i64,
}
