use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaimsDTO {
    username: String,
    exp: i64,
}
