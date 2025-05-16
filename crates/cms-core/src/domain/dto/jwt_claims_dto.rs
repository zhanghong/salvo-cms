use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct JwtClaimsDTO {
    pub uuid: String,
    pub user_id: String,
    pub user_type: String,
    pub token_type: String,
    pub exp: i64,
}
