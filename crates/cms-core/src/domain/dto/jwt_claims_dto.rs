use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct JwtClaimsDTO {
    pub uuid: String,
    pub user_id: i64,
    pub user_type: String,
    pub token_type: String,
    pub exp: i64,
}
