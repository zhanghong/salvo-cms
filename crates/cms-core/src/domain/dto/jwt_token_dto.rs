use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct JwtTokenDTO {
    pub token_type: String,
    pub token_value: String,
    pub expired_time: i64,
}
