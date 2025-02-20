use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct JwtClaimsDTO {
    pub user_id: i64,
    pub user_type: String,
    pub token_type: String,
    pub exp: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct JwtTokenDTO {
    pub token_type: String,
    pub token_value: String,
    pub expired_time: i64,
}
